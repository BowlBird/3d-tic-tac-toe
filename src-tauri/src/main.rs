// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod network_commands;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      network_commands::get_ip_address,
      network_commands::send_message,
      network_commands::receive_messages
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}