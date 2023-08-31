use std::net::UdpSocket;
use local_ip_address::local_ip;

#[tauri::command]
pub fn get_ip_address() -> String {
    let my_local_ip = local_ip().unwrap();
    my_local_ip.to_string()
}

#[tauri::command]
pub fn send_message(message: String, ip: String) {
    let host_ip = local_ip().unwrap().to_string();
    let socket = UdpSocket::bind(format!("{host_ip}:3334")).unwrap();
    socket.send_to(&message.clone().as_bytes(), format!("{ip}:3333")).unwrap();
}

#[tauri::command]
pub async fn receive_messages() -> String {
    let host_ip = local_ip().unwrap().to_string();
    let socket = UdpSocket::bind(format!("{host_ip}:3333")).unwrap();
    let mut buf = [0; 1024];
    let amount = socket.recv(&mut buf).unwrap();
    let buf = &mut buf[..amount];
    let result = String::from_utf8(buf.to_vec()).unwrap();
    result
}