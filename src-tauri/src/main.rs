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
use local_storage::Config;


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
    let config = Config::from_local();

    AppState {
      loaded_todo_lists: Mutex::new(
        config.get_task_vec()
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
  // println!("{:?}", locked_loaded_todo_lists);
  let cln = locked_loaded_todo_lists.clone();
  println!("{:?}", cln);
  // looks like input changes to front end only so perhaps related to serialisation, not backend
  cln
}


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
      prioritised_tasks: HashMap::new()
    };
    // insert this loaded todo to this hashmap
    locked_loaded_todo_lists.insert(new_tasklist.source.clone(), new_tasklist);
  }
  locked_loaded_todo_lists[&source].clone()
}

#[tauri::command]
fn recieve_task(state: tauri::State<AppState>, new_task: todo_lib::Task, source: String, hash: u64) {
  let mut locked_loaded_todo_lists = state
  .loaded_todo_lists
  .lock()
  .expect("Who is the othe user?");
  println!("{}", hash);

  // let mut identified_taskList = locked_loaded_todo_lists[&source];
  if let Some(identified_task_list) = locked_loaded_todo_lists.get_mut(&source) {
    identified_task_list.change_task(new_task);
    identified_task_list.save().unwrap();
  } else {
    // TODO: What should happen here?
    println!("could not get mutable version of the TaskList Prio hash")
  }
  println!("Recieve Task Finish")
  
}

// #[tauri::command]
// async fn my_custom_command(window: tauri::Window) {
//   window.eval("console.log(local_storage)");
//   println!("Window: {}", window.label());
// }
