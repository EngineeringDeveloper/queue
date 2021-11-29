use std::{convert::From, fs, io::{self, Read}, path::Path, str::FromStr};

use crate::Task;
use serde::{Deserialize, Serialize};

/// Task list struct is a container for a vec of tasks
/// It also does operations to keep track of changes sort and filter
///

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub source: String,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn from_file<'a, P : AsRef<Path> + std::fmt::Debug>(path: &'a P) -> Result<TaskList, io::Error> 
    where std::string::String: From<&'a P>
    {
        println!("{:?}", path);
        let mut todofile;
        match fs::File::open(path) {
            Ok(file) => todofile = file,
            Err(error) => {
                // dealing with this error if we can create the file path we will
                // and continue
                match error.kind() {
                    io::ErrorKind::NotFound => todofile = fs::File::create(path)?,
                    _ => return Err(error),
                }
            }
        }
        let mut contents = String::new();
        todofile.read_to_string(&mut contents)?;
        Ok(TaskList{
            source: String::from(path),
            tasks: vec_tasks_from_str(&contents),
        })
        // Ok(TaskList::from_str(&contents).expect("from Str should always return Ok"))
    }

    // pub fn to_file(self, path: &str) -> Result<(), io::Error> {
    //     let mut todofile;
    //     match fs::File::open(path) {
    //         Ok(file) => todofile = file,
    //         Err(error) => {
    //             // dealing with this error if we can create the file path we will
    //             // and continue
    //             match error.kind() {
    //                 io::ErrorKind::NotFound => todofile = fs::File::create(path)?,
    //                 _ => return Err(error),
    //             }
    //         }
    //     }
    // }


}

impl std::str::FromStr for TaskList {
    type Err = ();

    fn from_str(s: &str) -> Result<TaskList, ()> {
        Ok(TaskList {
            source: "".into(),
            tasks: vec_tasks_from_str(s)
            // {
            //     s.lines()
            //         .filter_map(|line| Task::from_str(line).ok())
            //         .collect()
            // }, // this always returns Ok
        })
    }
}

fn vec_tasks_from_str(s: &str) -> Vec<Task> {
            s.lines()
                .filter_map(|line| Task::from_str(line).ok())
                .collect()
    }
