// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::UdpSocket;
use local_ip_address::local_ip;
use network_commands::UnacknowledgedMessages;

pub mod network_commands;
pub mod logger;

pub struct Socket(UdpSocket); 

fn main() {
  tauri::Builder::default()
    .manage(Socket(open_socket()))
    .manage(UnacknowledgedMessages(Vec::new().into()))
    .invoke_handler(tauri::generate_handler![
      network_commands::get_encrypted_ip_address,
      network_commands::get_decrypted_ip_address,
      network_commands::receive_message,
      network_commands::send_message_encrypted_ip,
      network_commands::send_message_decrypted_ip,
      network_commands::encrypt_ip,
      network_commands::decrypt_ip,
      network_commands::message_watcher,
      logger::log
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn open_socket() -> UdpSocket {
  let ip = local_ip().unwrap().to_string();

  let base_socket = 50000;
  let mut iteration = 0;

  loop {
    let port = (base_socket + iteration).to_string();
    match UdpSocket::bind(format!("{ip}:{port}")) {
      Ok(socket) => {
        return socket
      }
      Err(_) => {
        iteration += 1;
        continue;
      }
    }
  }
}