import { reportReady } from "./api.js";

export function reportAfterPaint(label) {
  requestAnimationFrame(() => requestAnimationFrame(() => reportReady(label)));
}
