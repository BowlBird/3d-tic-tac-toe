use core::time;
use std::{sync::Mutex, thread};

use crate::Socket;

pub struct UnacknowledgedMessages(pub Mutex<Vec<(String, String)>>);

#[tauri::command]
pub fn get_decrypted_ip_address(socket: tauri::State<Socket>) -> String {
    let address = socket.0.local_addr().unwrap().to_string();
    println!("() -> {}", address);
    address
}

#[tauri::command]
pub fn get_encrypted_ip_address(socket: tauri::State<Socket>) -> String {
    let address = get_decrypted_ip_address(socket);
    let encrypted_address = encrypt_ip(address.clone()).unwrap();
    println!("({}) -> {}", address, encrypted_address);
    encrypted_address
}

#[tauri::command]
pub async fn send_message_decrypted_ip(
    message:String, 
    ip: String, 
    socket: tauri::State<'_, Socket>,
    unacknowledged_messages: tauri::State<'_, UnacknowledgedMessages>
) -> Result<(),()> {
    socket.0.send_to(message.as_bytes(),&ip).unwrap();
    println!("SENDING: {}\n\tTO: {}", &message, ip);
    unacknowledged_messages.0.lock().unwrap().push((message, ip));
    Ok(())
}

#[tauri::command]
pub async fn send_message_encrypted_ip(
    message: String, 
    encrypted_ip: String, 
    socket: tauri::State<'_, Socket>,
    unacknowledged_messages: tauri::State<'_, UnacknowledgedMessages>
) -> Result<(),()> {
    let ip = decrypt_ip(encrypted_ip.clone()).unwrap();
    println!("SENDING: {}\n\tTO: {} ({})", &message, encrypted_ip, decrypt_ip(encrypted_ip.clone()).unwrap());
    socket.0.send_to(message.as_bytes(),&ip).unwrap();
    unacknowledged_messages.0.lock().unwrap().push((message, ip));
    Ok(())
}

#[tauri::command]
pub async fn receive_message(socket: tauri::State<'_, Socket>, unacknowledged_messages: tauri::State<'_, UnacknowledgedMessages>) -> Result<String,()> {
    let mut buf = [0; 1024];
    let mut n;
    loop {
        n = socket.0.recv(&mut buf).unwrap();
        let buf = &buf[..n];
        let received_message: json::JsonValue = json::parse(String::from_utf8(buf.to_vec()).unwrap().as_str()).unwrap();
        println!("RECEIVED: {}\n\tFROM: {} ({})", &received_message, received_message["from"], decrypt_ip(received_message["from"].to_string()).unwrap());

        //if it is a confirmation
        if received_message["type"] == 0 {
            let mut index = 0;
            for message in unacknowledged_messages.0.lock().unwrap().iter() {
                if json::parse(message.0.as_str()).unwrap()["id"] ==  received_message["id"] {
                    break;
                }
                index += 1;
            }
            let removed = unacknowledged_messages.0.lock().unwrap().remove(index);
            println!("REMOVED FROM UNACKNOWLEDGED MESSAGES: {:?}", removed);
        }
        // otherwise send an acknowledgement
        // and break
        else {
            let message = json::parse(String::from_utf8(buf.to_vec()).unwrap().as_str()).unwrap();
            let response = json::object!{
                "from": encrypt_ip(socket.0.local_addr().unwrap().to_string()).unwrap(),
                "type": 0,
                "id": message["id"].clone()
            }.to_string();
            println!("SENDING ACKNOWLEDGEMENT: {} \n\tTO: {}", response, decrypt_ip(message["from"].to_string()).unwrap());
            socket.0.send_to(response.as_bytes(), decrypt_ip(message["from"].to_string()).unwrap()).unwrap();
            break;
        }
    }
    let buf = &buf[..n];
    Ok(String::from_utf8(buf.to_vec()).unwrap())
}

#[tauri::command]
pub async fn message_watcher(unacknowledged_messages: tauri::State<'_, UnacknowledgedMessages>, socket: tauri::State<'_, Socket>) -> Result<(),()> {
    loop {
        let mut iterator = 0;
        for message in unacknowledged_messages.0.lock().unwrap().iter() {
            match socket.0.send_to(message.0.as_bytes(), message.1.clone()) {
                Ok(_) => {}
                Err(_) => {
                    // this will occur when the ip becomes unreachable (disconnected)
                    unacknowledged_messages.0.lock().unwrap().remove(iterator);
                    /* TODO 
                    This may have issues when someone accidentally drops in the middle of a match due to network issues.
                    */
                }
            }
            iterator += 1;
        }
        println!("CURRENT UNACKNOWLEDGED MESSAGES: {:?}", unacknowledged_messages.0.lock().unwrap());
        thread::sleep(time::Duration::from_millis(2000))
    }
}

#[tauri::command]
pub fn encrypt_ip(ip: String) -> Result<String, String> {
    //xxx.xxx.xxx.xxx:xxxxx

    //extra space as it is unused.
    let chars = vec![' ', 'G', 'H', 'I', 'J'];
    let filtered_ip = ip.replace(".", ":");
    let mut result = String::new();
    let mut i = 0;
    for part in filtered_ip.split(":") {
        match part.parse::<u32>() {
            Ok(int_part) => {
                let character = chars[i];
                result = format!("{result}{character}{:X}", int_part)
            }
            Err(_) => {
                return Err(format!("'{ip}' not an encryptable IP"));
            }
        }
        i += 1;
    }
    result = result[1..].to_string();
    Ok(result)
}

#[tauri::command]
pub fn decrypt_ip(ip: String) -> Result<String, String> {
    //XXGXXHXXIXXJXX

    //easier formatting
    let chars = vec!['G', 'H', 'I', 'J'];
    let mut formatted_ip = ip.clone();
    for c in chars {
        formatted_ip = formatted_ip.replace(c, ":");
    }

    let mut result = String::new();
    for part in formatted_ip.split(":") {
        match u32::from_str_radix(part, 16) {
            Ok(integer) => {
                result = format!("{result}:{integer}");
            }
            Err(_) => {
                return Err(format!("'{ip}' not a decryptable IP"))
            }
        }
    }
    //remove leading ':'
    result = result[1..].to_string();

    //change first 3 ':' to '.'
    result = result.replacen(":", ".", 3);
    Ok(result)
}