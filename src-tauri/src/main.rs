#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod local_storage;
// mod todo;
mod update;

#[cfg(test)]
mod tests;

// use todo::{read_todo, LoadedTodo};
use std::collections::HashMap;
use std::sync::Mutex;


fn main() {
  tauri::Builder::default()
    .manage(AppState::read_local_state()) // gets ran on startup loads local state
    .invoke_handler(tauri::generate_handler![
      get_all_loaded_todo,
      get_todo,
      recieve_task
    ])
    // TODO also need to run a save function on window close
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Debug)]
struct AppState {
  // list of the todolists loaded, String key is the path to that list on disk
  loaded_todo_lists: Mutex<HashMap<String, todo_lib::TaskList>>,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.loaded_todo_lists.lock().unwrap().clone() == other.loaded_todo_lists.lock().unwrap().clone()
    }
}

impl AppState {
  fn read_local_state() -> AppState {
    let config = local_storage::load_local_config();

    AppState {
      loaded_todo_lists: Mutex::new(
        config
          .todo_txt_vec
          .into_iter()
          .filter_map(|path| {
            // todo_lib::from_file will create the file if it did not exist
            let task_list_result = todo_lib::TaskList::from_file(&path);
            match task_list_result {
              Ok(task_list) => Some((path, task_list)),
              Err(_) => None,
            }
          })
          .collect::<HashMap<String, todo_lib::TaskList>>(),
      ),
    }
  }
}

#[tauri::command]
fn get_all_loaded_todo(state: tauri::State<AppState>) -> HashMap<String, todo_lib::TaskList> {
  let locked_loaded_todo_lists = state
    .loaded_todo_lists
    .lock()
    .expect("Who is the othe user?");
  // println!("{:?}", locked_loaded_todo_lists.keys());
  locked_loaded_todo_lists.clone()
}

// #[tauri::command]
// fn todo_list_length(state: tauri::State<AppState>) -> usize {
//   let locked_loaded_todo_lists = state
//     .loaded_todo_lists
//     .lock()
//     .expect("Who is the othe user?");
//     locked_loaded_todo_lists.len()
// }

#[tauri::command]
fn get_todo(state: tauri::State<AppState>, source: String) -> todo_lib::TaskList {
  let mut locked_loaded_todo_lists = state
    .loaded_todo_lists
    .lock()
    .expect("Who is the othe user?");
  if !locked_loaded_todo_lists.contains_key(&source) {
    let new_tasklist = todo_lib::TaskList {
      source: source.clone(),
      tasks: vec![],
    };
    // insert this loaded todo to this hashmap
    locked_loaded_todo_lists.insert(new_tasklist.source.clone(), new_tasklist);
  }
  locked_loaded_todo_lists[&source].clone()
}

#[tauri::command]
fn recieve_task(state: tauri::State<AppState>, task: todo_lib::Task, source: String, identifier: String) {
  println!("{}/n{}/n{}", task, source, identifier);
  // let mut todo = &state.loaded_todo_lists[0];
  // let ident_task = &todo.todo_hash[&task.priority][index];

  // // change the task to the returned task
  // todo.todo_hash[&task.priority][index] = task;
  // // Write the changed to the file
  // todo.clone().write();
}

// #[tauri::command]
// async fn my_custom_command(window: tauri::Window) {
//   window.eval("console.log(local_storage)");
//   println!("Window: {}", window.label());
// }
