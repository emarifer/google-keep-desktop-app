#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu};
use tauri_plugin_window_state;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/* #[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
} */

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Salir").accelerator("cmdOrControl+Q");
    let submenu = Submenu::new("Archivo", Menu::new().add_item(quit));
    let menu = Menu::new().add_submenu(submenu);

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
 * DETALLES DE LA IMPLEMENTACIÓN. VER:
 * https://www.youtube.com/watch?v=uNDar53iwkU&list=PLmWYh0f8jKSjt9VC5sq2T3mFETasG2p2L
 * https://github.com/elibroftw/google-keep-desktop-app
 */
