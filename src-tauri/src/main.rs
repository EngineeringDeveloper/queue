#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod todo;
mod update;
mod local_storage;

use todo::{parse_todo, LoadedTodo,};
use todo_txt::Task;

fn main() {
  tauri::Builder::default()
    .manage(AppState::read_local_state()) // gets ran on startup loads local state
    .invoke_handler(tauri::generate_handler![parse_todo, get_todo, todo_list_length])
    // TODO also need to run a save function on window close
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


struct AppState {
  todo_list: Vec<LoadedTodo>
}

impl AppState {
  fn read_local_state() -> AppState {
    let config = local_storage::load_local_config();

    AppState{
      todo_list: config.todo_txt_vec.into_iter().map(|path| {
      LoadedTodo::from_path(&path)
    }).collect()
    }
  }

}

#[tauri::command]
fn todo_list_length(state: tauri::State<AppState>) -> usize {
  state.todo_list.len()
}

#[tauri::command]
fn get_todo(state: tauri::State<AppState>, index: usize) -> Vec<Task> {
  state.todo_list[index].task_vec.clone()
}


#[tauri::command]
async fn my_custom_command(window: tauri::Window) {
  window.eval("console.log(local_storage)");
  println!("Window: {}", window.label());
}
