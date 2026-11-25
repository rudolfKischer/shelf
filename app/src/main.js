import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { reportAfterPaint } from "./lib/timing.js";

mount(App, { target: document.getElementById("app") });
reportAfterPaint("shell-painted");
