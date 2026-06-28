/**
 * Captures README screenshots from the real Svelte UI (demo mode).
 *
 * Usage:
 *   npm run dev          (terminal 1)
 *   npm run capture-screenshots
 */
import { mkdir } from "node:fs/promises";
import { spawn } from "node:child_process";
import { join } from "node:path";
import { chromium } from "playwright";

const ROOT = join(import.meta.dirname, "..");
const OUT_DIR = join(ROOT, "docs", "screenshots");
const BASE = "http://localhost:1420";

const WELCOME_VIEWPORT = { width: 880, height: 660 };
/** 4:3 — enough width for preview + right sidebar without clipping */
const WORKSPACE_VIEWPORT = { width: 1320, height: 990 };

async function waitForServer(maxMs = 45000) {
  const start = Date.now();
  while (Date.now() - start < maxMs) {
    try {
      const res = await fetch(BASE);
      if (res.ok) return;
    } catch {
      /* retry */
    }
    await new Promise((r) => setTimeout(r, 500));
  }
  throw new Error("Dev server not running — start it with: npm run dev");
}

function startDevServer() {
  const child = spawn("npm", ["run", "dev"], {
    cwd: ROOT,
    stdio: "ignore",
    detached: true,
  });
  child.unref();
  return child;
}

async function captureWorkspace(page, mode, filename) {
  await page.setViewportSize(WORKSPACE_VIEWPORT);
  await page.goto(`${BASE}/?screenshot=${mode}`, { waitUntil: "networkidle" });
  await page.waitForSelector(`[data-screenshot-ready='${mode}']`, { timeout: 15000 });

  if (mode === "workspace-video") {
    await page.waitForSelector("video.preview-media", { timeout: 15000 });
    await page.waitForFunction(() => {
      const video = document.querySelector("video.preview-media");
      return video instanceof HTMLVideoElement && video.readyState >= 2 && video.duration > 0;
    });
    await page.waitForTimeout(700);
  } else {
    await page.waitForSelector("img.preview-media", { timeout: 15000 });
    await page.waitForTimeout(400);
  }

  await page.locator(".app-shell").screenshot({
    path: join(OUT_DIR, filename),
  });
}

async function main() {
  await mkdir(OUT_DIR, { recursive: true });

  let startedDev = false;
  try {
    await waitForServer(2000);
  } catch {
    startDevServer();
    startedDev = true;
    await waitForServer();
  }

  const browser = await chromium.launch();
  const page = await browser.newPage({ deviceScaleFactor: 2 });

  await page.setViewportSize(WELCOME_VIEWPORT);
  await page.goto(`${BASE}/?screenshot=welcome`, { waitUntil: "networkidle" });
  await page.waitForSelector("[data-screenshot-ready='welcome']", { timeout: 15000 });
  await page.locator(".welcome-card").screenshot({
    path: join(OUT_DIR, "welcome.png"),
  });

  await captureWorkspace(page, "workspace", "workspace.png");
  await captureWorkspace(page, "workspace-video", "workspace-video.png");

  await browser.close();
  console.log("Screenshots saved to docs/screenshots/");

  if (startedDev) {
    console.log("Note: a background dev server may still be running on :1420");
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
