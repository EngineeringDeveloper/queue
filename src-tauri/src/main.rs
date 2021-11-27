#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod local_storage;
mod todo;
mod update;

use todo::{read_todo, LoadedTodo};
use todo_txt::Task;
use std::collections::HashMap;
use todo_lib;


fn main() {
  tauri::Builder::default()
    .manage(AppState::read_local_state()) // gets ran on startup loads local state
    .invoke_handler(tauri::generate_handler![
      read_todo,
      get_todo,
      todo_list_length,
      recieve_task
    ])
    // TODO also need to run a save function on window close
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

struct AppState {
  todo_list: Vec<LoadedTodo>,
}

impl AppState {
  fn read_local_state() -> AppState {
    let config = local_storage::load_local_config();

    AppState {
      todo_list: config
        .todo_txt_vec
        .into_iter()
        .map(|path| LoadedTodo::from_path(&path))
        .collect(),
    }
  }
}

#[tauri::command]
fn todo_list_length(state: tauri::State<AppState>) -> usize {
  state.todo_list.len()
}

#[tauri::command]
fn get_todo(state: tauri::State<AppState>, index: usize) -> HashMap<u8, Vec<Task>> {
  state.todo_list[index].todo_hash.clone()
}

#[tauri::command]
fn recieve_task(state: tauri::State<AppState>, task: Task, index: usize) {
  let mut todo = &state.todo_list[0];
  let ident_task = &todo.todo_hash[&task.priority][index];
  println!("{}/n/n{:?}", index, ident_task);
  
  // change the task to the returned task
  todo.todo_hash[&task.priority][index] = task;
  // Write the changed to the file
  todo.clone().write();
}

// #[tauri::command]
// async fn my_custom_command(window: tauri::Window) {
//   window.eval("console.log(local_storage)");
//   println!("Window: {}", window.label());
// }
