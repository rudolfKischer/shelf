const INPUT_CONTROLLER_FULL = "ControllerFull";
const INPUT_CONTROLLER_PARTIAL = "ControllerPartial";

function runtimeGlyph(game) {
  if (game.wine?.blocked) return "blocked";
  if (game.features.platforms.mac) return "mac";
  return "windows";
}

function inputGlyphs(input) {
  if (input === INPUT_CONTROLLER_FULL) return ["gamepad"];
  if (input === INPUT_CONTROLLER_PARTIAL) return ["gamepad", "keyboard"];
  return ["keyboard"];
}

export function glyphsFor(game) {
  const list = [runtimeGlyph(game), ...inputGlyphs(game.features.input)];
  if (game.features.local_multiplayer) list.push("people");
  if (game.features.online_multiplayer) list.push("globe");
  return list;
}

export function installGlyph(game) {
  return game.copies.length > 0 ? "installed" : "notInstalled";
}
