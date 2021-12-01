use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::de;

// import for writing files
use std::{
  fs::{create_dir_all, File},
  io::Write, path::{PathBuf}, collections::HashMap,
};
// use std::io::{Write, Read};

const NAME: &str = "Queue"; // env!("CARGO_PKG_NAME"); // Hopefully no more name changes but this keeps it simple
const AUTHOR: &str = "EngineeringDeveloper";

//
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Config {
  pub todo_txt_vec: Vec<String>,
}

impl Config {
  
  pub fn from_path(path: &str) -> Config {
    // TODO Error handeling
    let file = load_file(path);
    de::from_reader(file).unwrap()
  }

  pub fn from_local() -> Config {
    // TODO Error handeling
    de::from_reader(get_local_config_file()).unwrap()
  }

  
  pub fn save_local_config_to_path(self) -> Result<(), std::io::Error> {
    // TODO Error handeling
    let mut file = self::get_local_config_file();
    file.write_all(serde_json::to_string(&self).unwrap().as_bytes())
  }

  

  pub fn get_taskVec(self) -> HashMap<String, todo_lib::TaskList>{
    self.todo_txt_vec
          .into_iter()
          .filter_map(|path| {
            // todo_lib::from_file will create the file if it did not exist
            let task_list_result = todo_lib::TaskList::from_file(&path);
            match task_list_result {
              Ok(task_list) => Some((path, task_list)),
              Err(_) => None,
            }
          })
          .collect::<HashMap<String, todo_lib::TaskList>>()
  }
}

fn get_local_config_file() -> File {
  // get the local config location
  // TODO Error handeling
  let path;
  if cfg!(debug_assertions) {
    path = PathBuf::from("..\\testFiles\\Queue.config");
  } else {
    let project_dir = ProjectDirs::from("", AUTHOR, NAME).unwrap();
    let dir_path = project_dir.config_dir();
    create_dir_all(&dir_path).unwrap();
    path = dir_path.join("Queue.config");
  }
  load_file(&path.to_str().expect("Path should always be convertable to str"))
}

fn load_file(path: &str) -> File {
  // TODO Error handeling
  // Creates file if not existing
  // but does not create the subfolders
  // let mut file;
  // File::open(&path)
  let mut file;
  match File::open(&path) {
    Ok(opened_file) => file = opened_file,
    Err(_) => {
      // file does not exist
      file = File::create(&path).unwrap();
      file.write_all(
        serde_json::to_string(&Config {
          todo_txt_vec: vec!["".into()],
        })
        .unwrap()
        .as_bytes(),
      ).unwrap();
    }
  }
  file
}