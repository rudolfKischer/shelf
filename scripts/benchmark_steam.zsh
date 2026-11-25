#!/bin/zsh
zmodload zsh/datetime

SCRIPT_DIRECTORY="${0:A:h}"
FLAG_SETS_FILE="${FLAG_SETS_FILE:-$SCRIPT_DIRECTORY/steam_flag_sets.txt}"
STEAM_APP="${STEAM_APP:-/Applications/Steam.app}"
STEAM_ROOT="${STEAM_ROOT:-$HOME/Library/Application Support/Steam}"
RUNS_PER_VARIANT="${RUNS_PER_VARIANT:-2}"
QUIT_GRACE_SECONDS="${QUIT_GRACE_SECONDS:-20}"
QUIT_TIMEOUT_SECONDS="${QUIT_TIMEOUT_SECONDS:-60}"
READY_TIMEOUT_SECONDS="${READY_TIMEOUT_SECONDS:-120}"
SETTLE_SECONDS="${SETTLE_SECONDS:-25}"
POLL_INTERVAL_SECONDS="${POLL_INTERVAL_SECONDS:-0.25}"
PAUSE_BETWEEN_RUNS_SECONDS="${PAUSE_BETWEEN_RUNS_SECONDS:-3}"
CLIENT_PROCESS_PATTERN="Steam.AppBundle/Steam/Contents"
RUNNING_PROCESS_PATTERN="Steam.AppBundle/Steam/Contents/MacOS/steam_osx"
READY_PATTERN='RecvMsgClientLogOnResponse\(\) : processing complete'
STARTUP_TIME_PATTERN='System startup time'
LAUNCH_LINE_PATTERN='Steam Client launched with'
CONNECTION_LOG="$STEAM_ROOT/logs/connection_log.txt"
CONSOLE_LOG="$STEAM_ROOT/logs/console_log.txt"
BOOTSTRAP_LOG="$STEAM_ROOT/logs/bootstrap_log.txt"

steam_is_running() {
  pgrep -f "$RUNNING_PROCESS_PATTERN" >/dev/null
}

wait_until_stopped() {
  local timeout="$1" started=$EPOCHREALTIME
  while steam_is_running; do
    (( EPOCHREALTIME - started > timeout )) && return 1
    sleep "$POLL_INTERVAL_SECONDS"
  done
  return 0
}

quit_steam() {
  steam_is_running || return 0
  "$STEAM_APP/Contents/MacOS/steam_osx" -shutdown >/dev/null 2>&1
  wait_until_stopped "$QUIT_GRACE_SECONDS" && return 0
  osascript -e 'tell application id "com.valvesoftware.steam" to quit' >/dev/null 2>&1
  wait_until_stopped "$QUIT_TIMEOUT_SECONDS"
}

line_count() {
  wc -l < "$1" | tr -d ' '
}

lines_after() {
  tail -n +"$(( $2 + 1 ))" "$1"
}

wait_for_new_match() {
  local file="$1" after_line="$2" pattern="$3" timeout="$4" started=$EPOCHREALTIME
  while ! lines_after "$file" "$after_line" | grep -q -E "$pattern"; do
    (( EPOCHREALTIME - started > timeout )) && { echo "timeout"; return 1; }
    sleep "$POLL_INTERVAL_SECONDS"
  done
  printf "%.2f" "$(( EPOCHREALTIME - started ))"
}

client_memory_megabytes() {
  ps -axo rss,command | awk -v pattern="$CLIENT_PROCESS_PATTERN" 'index($0, pattern) {sum += $1} END {print int(sum / 1024)}'
}

client_process_count() {
  pgrep -f "$CLIENT_PROCESS_PATTERN" | wc -l | tr -d ' '
}

reported_startup_seconds() {
  lines_after "$CONSOLE_LOG" "$1" | grep "$STARTUP_TIME_PATTERN" | tail -1 | sed -E 's/.*: ([0-9.]+) seconds.*/\1/'
}

forwarded_arguments() {
  lines_after "$BOOTSTRAP_LOG" "$1" | grep "$LAUNCH_LINE_PATTERN" | tail -1 | sed -E "s/.*launched with: *//"
}

start_steam() {
  local flags="$1"
  [[ -z "$flags" ]] && { open -a "$STEAM_APP"; return; }
  open -a "$STEAM_APP" --args ${=flags}
}

run_variant() {
  local name="$1" flags="$2" run_number="$3"
  quit_steam || { echo "$name\t$run_number\tcould not quit steam" ; return 1; }
  sleep "$PAUSE_BETWEEN_RUNS_SECONDS"
  local connection_mark console_mark bootstrap_mark
  connection_mark=$(line_count "$CONNECTION_LOG")
  console_mark=$(line_count "$CONSOLE_LOG")
  bootstrap_mark=$(line_count "$BOOTSTRAP_LOG")
  start_steam "$flags"
  local logged_on_seconds
  logged_on_seconds=$(wait_for_new_match "$CONNECTION_LOG" "$connection_mark" "$READY_PATTERN" "$READY_TIMEOUT_SECONDS")
  sleep "$SETTLE_SECONDS"
  printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n" "$name" "$run_number" "$logged_on_seconds" \
    "$(reported_startup_seconds "$console_mark")" "$(client_memory_megabytes)" "$(client_process_count)" \
    "$(forwarded_arguments "$bootstrap_mark")" "$flags"
}

run_all_variants() {
  local run_number name flags
  for run_number in $(seq 1 "$RUNS_PER_VARIANT"); do
    while IFS='|' read -r name flags; do
      [[ -z "$name" ]] && continue
      run_variant "$name" "$flags" "$run_number"
    done < "$FLAG_SETS_FILE"
  done
}

restore_normal_steam() {
  quit_steam
  sleep "$PAUSE_BETWEEN_RUNS_SECONDS"
  start_steam ""
}

printf "variant\trun\tlogged_on_s\treported_startup_s\tmemory_mb\tprocesses\tforwarded_args\tflags\n"
run_all_variants
restore_normal_steam
