use std::{
  collections::HashMap,
  fs,
  io::{self, BufRead, Write},
  iter::FromIterator,
  str::FromStr,
};
use todo_txt::Task;

pub struct LoadedTodo {
  pub path: String,
  pub todo_hash: HashMap<u8, Vec<Task>>, // hashmap with keys as priorty and content vector of tasks
}

impl LoadedTodo {
  pub fn from_path(path: &str) -> LoadedTodo {
    LoadedTodo {
      path: String::from(path),
      todo_hash: task_vec_to_hashmap(read_todo(path)),
    }
  }

  pub fn write(self) {
    // TODO Error Handeling
    let mut file = fs::File::open(self.path).unwrap();
    let mut lines = vec![];
    for vec in self.todo_hash.values() {
      lines.append(&mut vec.clone());
    }
    for line in lines {
      file.write_all(&line.to_string().as_bytes());
    }
  }
}

fn task_vec_to_hashmap(task_vec: Vec<Task>) -> HashMap<u8, Vec<Task>> {
  let mut task_map = HashMap::new();
  for task in task_vec.iter() {
    task_map
      .entry(task.priority)
      .or_insert_with(Vec::new)
      .push(task.clone())
  }
  task_map
}

#[tauri::command]
pub fn read_todo(path: &str) -> Vec<Task> {
  let todofile = io::BufReader::new(fs::File::open(path).unwrap());
  Vec::from_iter(todofile.lines().map(|line| {
    if let Ok(value) = line {
      Task::from_str(&value).unwrap()
    } else if let Err(error) = line {
      Task::from_str(&format!("{}", error)).unwrap()
    } else {
      Task::from_str("Failed Task").unwrap()
    }
  }))
}
