use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use serde_json;


// import for writing files
use std::{fs::{File, create_dir_all}, io::Write};
// use std::io::{Write, Read};

const NAME: &str = "Queue"; // env!("CARGO_PKG_NAME"); // Hopefully no more name changes but this keeps it simple
const AUTHOR: &str = "EngineeringDeveloper";
  

// 
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub todo_txt_vec: Vec<String>,
}


/// get the local config location
pub fn get_local_config() -> File {
    // TODO Error handeling
    let project_dir = ProjectDirs::from("", AUTHOR, NAME)
    .unwrap();
    let path = project_dir.config_dir();
    create_dir_all(&path).unwrap();
    let path  = path.join("Queue.config");

    match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                // file does not exist
                println!("{}", &path.to_str().unwrap());
                let file = File::create(&path).unwrap();
                // save empty config
                save_local_config(Config{
                    todo_txt_vec: vec!["".into()]
                });
                file
            }
        }
}

pub fn save_local_config(config: Config) -> Result<(), std::io::Error> {
    // TODO Error handeling
    let mut file = get_local_config();
    file.write_all(serde_json::to_string(&config).unwrap().as_bytes())
}
