"use strict";

/**
 * Courseo SecureWatch - Hardened Desktop Launcher (Electron / Tauri Bridge)
 *
 * Enforces OS-level hardware display affinity and window blinding:
 *  - macOS: NSWindowSharingNone (sharingType = 0)
 *  - Windows: WDA_EXCLUDEFROMCAPTURE (0x11)
 *  - Suppresses DevTools and screen-capture keyboard shortcuts
 */

const CAPTURE_SHORTCUTS = Object.freeze([
  "PrintScreen",
  "Alt+PrintScreen",
  "CommandOrControl+Shift+3",
  "CommandOrControl+Shift+4",
  "CommandOrControl+Shift+5",
  "CommandOrControl+Shift+I",
  "CommandOrControl+Alt+I",
  "F12"
]);

const OS_DISPLAY_PROTECTION_FLAGS = Object.freeze({
  darwin: 0,
  win32: 0x11,
  android: 0x00002000
});

/**
 * Creates and hardens the primary Courseo SecureWatch desktop window.
 *
 * @param {object} electronApi - Electron module ({ app, BrowserWindow, globalShortcut })
 * @param {string} targetUrl - Authenticated Courseo playback URL
 * @returns {object} Hardened BrowserWindow instance
 */
function createSecureDesktopWindow(electronApi, targetUrl) {
  const { BrowserWindow, globalShortcut } = electronApi;

  const mainWindow = new BrowserWindow({
    width: 1280,
    height: 720,
    show: false,
    autoHideMenuBar: true,
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
      devTools: false,
      webSecurity: true
    }
  });

  // Enforce OS-level hardware capture blinding (NSWindowSharingNone / WDA_EXCLUDEFROMCAPTURE)
  mainWindow.setContentProtection(true);

  // Block DevTools and capture shortcuts at the webContents input layer
  mainWindow.webContents.on("before-input-event", (event, input) => {
    const key = String(input.key || "").toLowerCase();
    const isDevToolsCombo =
      key === "f12" ||
      ((input.control || input.meta) && input.shift && key === "i") ||
      (input.meta && input.alt && key === "i");
    const isCaptureKey =
      key === "printscreen" ||
      (input.meta && input.shift && (key === "3" || key === "4" || key === "5"));

    if (isDevToolsCombo || isCaptureKey) {
      event.preventDefault();
    }
  });

  mainWindow.webContents.on("devtools-opened", () => {
    mainWindow.webContents.closeDevTools();
  });

  // Suppress global screen capture shortcuts while the secure player is active
  if (globalShortcut && typeof globalShortcut.register === "function") {
    for (const accelerator of CAPTURE_SHORTCUTS) {
      try {
        globalShortcut.register(accelerator, () => {
          // Intentionally suppressed to prevent OS capture hotkeys
        });
      } catch (_err) {
        // Ignore platform-unsupported accelerators
      }
    }
  }

  mainWindow.once("ready-to-show", () => {
    mainWindow.setContentProtection(true);
    mainWindow.show();
  });

  mainWindow.on("closed", () => {
    if (globalShortcut && typeof globalShortcut.unregisterAll === "function") {
      globalShortcut.unregisterAll();
    }
  });

  if (targetUrl) {
    mainWindow.loadURL(targetUrl);
  }

  return mainWindow;
}

if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    CAPTURE_SHORTCUTS,
    OS_DISPLAY_PROTECTION_FLAGS,
    createSecureDesktopWindow
  };
}
