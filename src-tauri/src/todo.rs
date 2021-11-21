use todo_txt::Task;
use std::{fs, io::{self, BufRead}, str::FromStr, iter::FromIterator};


#[tauri::command]
pub fn parse_todo(path: &str) -> Vec<Task>{
    let path = "G:\\My Drive\\todo.txt";
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