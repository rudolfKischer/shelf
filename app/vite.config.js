import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

const DEV_SERVER_PORT = 1420;
const BUILD_TARGET = "safari15";

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: { port: DEV_SERVER_PORT, strictPort: true },
  build: { target: BUILD_TARGET },
});
