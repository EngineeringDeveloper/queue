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
    .manage(AppState::get_local_state())
    .invoke_handler(tauri::generate_handler![print_from_rust, parse_todo, my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


struct AppState {
  todo_list: Vec<LoadedTodo>
}

impl AppState {
  fn get_local_state() -> AppState {

    AppState{
      todo_list: vec![LoadedTodo {
        path: "Path".into(),
        task_vec: vec![Task::from_str("Test").unwrap()]
      }]
    }
  }
  
  fn from_path(path: &str) {
      // opens the config file and loads upp the app state from it
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
