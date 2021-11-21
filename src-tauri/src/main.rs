#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod todo;
use todo::parse_example;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![print_from_rust, parse_example])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Example Tauri command
#[tauri::command]
fn print_from_rust() {
  println!("I was generated in Rust!");
  parse_example("path");
}
