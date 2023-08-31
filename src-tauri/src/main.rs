// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod network_commands;

fn main() {
  tauri::Builder::default()
    .manage(network_commands::Listener(None.into()))
    .manage(network_commands::Sender(None.into()))
    .invoke_handler(tauri::generate_handler![
      network_commands::get_ip_address,
      network_commands::allow_connections,
      network_commands::connect,
      network_commands::send_message,
      network_commands::receive_messages
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}