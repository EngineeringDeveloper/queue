#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod todo;
mod update;
mod localStorage;

use todo::{parse_todo, LoadedTodo,};
use todo_txt::Task;
use std::str::FromStr;

fn main() {
  localStorage::save_local_config(localStorage::Config{
    todo_txt_vec: vec!["TEST".into()]
  });
  tauri::Builder::default()
    .manage(AppState::read_local_state()) // gets ran on startup
    .invoke_handler(tauri::generate_handler![print_from_rust, parse_todo, my_custom_command])
    // TODO also need to run a save function on window close
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


struct AppState {
  todo_list: Vec<LoadedTodo>
}

impl AppState {
  fn read_local_state() -> AppState {
    let config = localStorage::load_local_config();

    AppState{
      todo_list: config.todo_txt_vec.into_iter().map(|path| {
      LoadedTodo::from_path(&path)
    }).collect()
    }
  }
}

// Example Tauri command
#[tauri::command]
fn print_from_rust() {
  println!("I was generated in Rust!");
}

#[tauri::command]
async fn my_custom_command(window: tauri::Window) {
  window.eval("console.log(localStorage)");
  println!("Window: {}", window.label());
}
