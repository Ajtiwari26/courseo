#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[cfg(target_os = "macos")]
fn apply_os_content_protection(window: &tauri::WebviewWindow) {
    use cocoa::appkit::NSWindowSharingType;
    use objc::{msg_send, sel, sel_impl};

    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_win = ns_window as cocoa::base::id;
            let _: () = msg_send![ns_win, setSharingType: NSWindowSharingType::NSWindowSharingNone];
        }
    }
}

#[cfg(target_os = "windows")]
fn apply_os_content_protection(window: &tauri::WebviewWindow) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE,
    };

    if let Ok(hwnd) = window.hwnd() {
        unsafe {
            let _ = SetWindowDisplayAffinity(HWND(hwnd.0), WDA_EXCLUDEFROMCAPTURE);
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn apply_os_content_protection(_window: &tauri::WebviewWindow) {}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(main_window) = app.get_webview_window("main") {
                apply_os_content_protection(&main_window);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Courseo SecureWatch Tauri desktop application");
}