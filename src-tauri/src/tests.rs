use crate::*;
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};
use std::fs::canonicalize;
use todo_lib::TaskList;

#[test]
fn test_AppState() {
  let app_state = AppState::read_local_state();
  //   let locked_loaded_todo_lists = app_state
  //     .loaded_todo_lists
  //     .lock()
  //     .expect("Who is the othe user?");
  //   println!("{:?}", locked_loaded_todo_lists);

  let path = "..\\testFiles\\todo.txt";
  let path_2 = "..\\testFiles\\otherTodo.txt";
  let correct_state = AppState {
    loaded_todo_lists: Mutex::new(HashMap::from([
      (
        path.to_owned(),
        TaskList::from_file(&path.to_owned()).expect("why not"),
      ),
      (
        path_2.to_owned(),
        TaskList::from_file(&path_2.to_owned()).expect("why not"),
      ),
    ])),
  };
  assert_eq!(app_state, correct_state);
}

#[test]
fn test_local_storage_config() {
  // Test load local config
  let config = local_storage::Config {
    todo_txt_vec: vec!["..\\testFiles\\todo.txt".to_owned(), "..\\testFiles\\otherTodo.txt".to_owned()],
  };
  assert_eq!(config, local_storage::Config::from_local());
}

