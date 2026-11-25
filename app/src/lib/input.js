import settings from "./settings.json";

const { gamepad: pad, keys, repeatingActions } = settings;
const TYPING_TAGS = new Set(["INPUT", "TEXTAREA", "SELECT"]);

function stickDirections(axes) {
  const horizontal = axes[pad.stickAxes.horizontal] ?? 0;
  const vertical = axes[pad.stickAxes.vertical] ?? 0;
  const found = [];
  if (horizontal < -pad.deadzone) found.push("left");
  if (horizontal > pad.deadzone) found.push("right");
  if (vertical < -pad.deadzone) found.push("up");
  if (vertical > pad.deadzone) found.push("down");
  return found;
}

function activeActions(gamepad) {
  const fromButtons = Object.entries(pad.buttons)
    .filter(([index]) => gamepad.buttons[index]?.pressed)
    .map(([, action]) => action);
  return new Set([...fromButtons, ...stickDirections(gamepad.axes)]);
}

export function createInput(onAction) {
  const heldSince = new Map();
  let frame = 0;

  function shouldFire(action, now) {
    const record = heldSince.get(action);
    if (!record) {
      heldSince.set(action, { first: now, last: now });
      return true;
    }
    if (!repeatingActions.includes(action)) return false;
    const waited = now - record.first >= pad.repeatDelayMs;
    const due = now - record.last >= pad.repeatIntervalMs;
    if (waited && due) record.last = now;
    return waited && due;
  }

  function poll(now) {
    const current = new Set();
    for (const gamepad of navigator.getGamepads()) {
      if (gamepad) activeActions(gamepad).forEach((action) => current.add(action));
    }
    for (const action of [...heldSince.keys()]) if (!current.has(action)) heldSince.delete(action);
    current.forEach((action) => shouldFire(action, now) && onAction(action));
    frame = requestAnimationFrame(poll);
  }

  function onKeyDown(event) {
    if (TYPING_TAGS.has(event.target?.tagName)) return;
    const action = keys[event.key];
    if (!action || event.metaKey || event.ctrlKey) return;
    event.preventDefault();
    onAction(action);
  }

  window.addEventListener("keydown", onKeyDown);
  frame = requestAnimationFrame(poll);
  return () => {
    window.removeEventListener("keydown", onKeyDown);
    cancelAnimationFrame(frame);
  };
}
