use crate::Socket;

#[tauri::command]
pub fn get_decrypted_ip_address(socket: tauri::State<Socket>) -> String {
    socket.0.local_addr().unwrap().to_string()
}

#[tauri::command]
pub fn get_encrypted_ip_address(socket: tauri::State<Socket>) -> String {
    encrypt_ip(socket.0.local_addr().unwrap().to_string()).unwrap()
}

#[tauri::command]
pub async fn send_message_decrypted_ip(message:String, ip: String, socket: tauri::State<'_, Socket>) -> Result<(),()> {
    socket.0.send_to(message.as_bytes(),ip).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn send_message_encrypted_ip(message:String, encrypted_ip: String, socket: tauri::State<'_, Socket>) -> Result<(),()> {
    let ip = decrypt_ip(encrypted_ip).unwrap();
    socket.0.send_to(message.as_bytes(),ip).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn receive_message(socket: tauri::State<'_, Socket>) -> Result<String,()> {
    let mut buf = [0; 1024];
    socket.0.recv(&mut buf).unwrap();
    Ok(String::from_utf8(buf.to_vec()).unwrap())
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