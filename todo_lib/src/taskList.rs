use std::{
    fs,
    io::{self, Read},
    str::FromStr,
};

use crate::Task;
use serde::{Deserialize, Serialize};

/// Task list struct is a container for a vec of tasks
/// It also does operations to keep track of changes sort and filter
///
///

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub source: Option<String>,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn from_file(path: &str) -> Result<TaskList, io::Error> {
        let mut todofile = fs::File::open(path)?;
        let mut contents = String::new();
        todofile.read_to_string(&mut contents)?;
        Ok(TaskList::from_str(&contents).expect("from Str should always return Ok"))
    }
}

impl std::str::FromStr for TaskList {
    type Err = ();

    fn from_str(s: &str) -> Result<TaskList, ()> {
        Ok(TaskList {
            source: None,
            tasks: {
                s.lines()
                    .filter_map(|line| Task::from_str(line).ok())
                    .collect()
            }, // this always returns Ok
        })
    }
}
