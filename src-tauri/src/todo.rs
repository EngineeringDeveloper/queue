use todo_txt::Task;
use std::{fs, io::{self, BufRead}, str::FromStr, iter::FromIterator};

struct AppState {
    todo_list: Vec<LoadedTodo>
}

impl AppState {
    fn from_file() {
        // opens the config file and loads upp the app state from it
    }
}

struct LoadedTodo {
    path: String,
    task_vec: Vec<Task>,
}

impl LoadedTodo {

    fn from_string(path: &str) -> LoadedTodo {
        LoadedTodo {
            path,
            task_vec: parse_todo(path)
        }
    }
    
}


#[tauri::command]
pub fn parse_todo(path: &str) -> Vec<Task>{
    let todofile = io::BufReader::new(fs::File::open(path).unwrap());
    Vec::from_iter(todofile.lines().map(|line| {
        if let Ok(value) = line {
            Task::from_str(&value).unwrap()
        }
        else if let Err(error) = line {
            Task::from_str(&format!("{}", error)).unwrap()
        }
        else {
            Task::from_str("Failed Task").unwrap()
        }
    }))
}