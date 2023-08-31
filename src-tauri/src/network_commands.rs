use std::{
    net::{TcpListener, TcpStream},
    io::{self, Write, Read},
    thread, sync::Mutex,
};
use local_ip_address::local_ip;

//for state
pub struct Listener(pub Mutex<Option<TcpListener>>);
pub struct Sender(pub Mutex<Option<TcpStream>>);

#[tauri::command]
pub fn get_ip_address() -> String {
    let my_local_ip = local_ip().unwrap();
    my_local_ip.to_string()
}

#[tauri::command]
pub fn allow_connections(state: tauri::State<Listener>) {
    println!("Listening!");
    let ip = get_ip_address();
    let listener = TcpListener::bind(format!("{ip}:3333")).unwrap();
    let mut listener_state = state.0.lock().unwrap();
    *listener_state = Some(listener);
    // thread::spawn(|| {
    //     let ip = get_ip_address();
    //     let listener = TcpListener::bind(format!("{ip}:3333")).unwrap();

    //     for stream in listener.incoming() {
    //         let mut stream = stream.unwrap();
    //         let mut buffer = [0; 1024];

    //         loop {
    //             let n = stream.read(&mut buffer[..]).unwrap();
    //             if n == 0 {
    //                 // Connection was closed
    //                 break;
    //             }
    //             let result = String::from_utf8(buffer[..n].to_vec()).unwrap();
    //             println!("Received: {:?}", &result);
    //         }
    //     }
    // });
}

#[tauri::command]
pub fn connect(ip: String, state: tauri::State<Sender>) {
    println!("{}", format!("{ip}:3333"));
    let sender = TcpStream::connect(format!("{ip}:3333")).unwrap();
    let mut sender_state = state.0.lock().unwrap();
    *sender_state = Some(sender);
    // thread::spawn(move || {
    //     loop {
    //         let mut buffer = String::new();
    //         io::stdin().read_line(&mut buffer).unwrap();

    //         sender.write_all(buffer.as_bytes()).unwrap();
    //         sender.flush().unwrap();
    //     }
    // });
}

#[tauri::command]
pub fn send_message(message: String, state: tauri::State<Sender>) {
    println!("Sending: {}", message);
    let mut sender_state = state.0.lock().unwrap();

    let sender_state = sender_state.as_mut();
    match sender_state {
        Some(sender) => {
            sender.write_all(message.as_bytes()).unwrap();
            //sender.flush().unwrap();
        }
        None => {println!("Send Failed with error")}
    }
}

#[tauri::command]
pub fn receive_messages(state: tauri::State<Listener>) -> String {
    let mut result = String::new();

    let listener_state = state.0.lock().unwrap();
    let listener_state = listener_state.as_ref();
    println!("HERE");

    match listener_state {
        Some(listener) => {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 1024];
        
                    let n = stream.read(&mut buffer[..]).unwrap();
                    // if n == 0 {
                    //     // Connection was closed
                    //     break;
                    // }
                    let inner_result = String::from_utf8(buffer[..n].to_vec()).unwrap();
                    result = format!("{result}{inner_result}");
                    break;
            }
            result
        }
        None => {
            println!("Error with receiving_messages!");
            String::new()
        }
    }
}