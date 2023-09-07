use colored::Colorize;

#[tauri::command]
pub fn log(string: String) {
    println!("{}", string.yellow());
}