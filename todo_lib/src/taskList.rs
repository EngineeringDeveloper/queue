use crate::Task;
use serde::{Serialize, Deserialize};

/// Task list struct is a container for a vec of tasks
/// It also does operations to keep track of changes sort and filter
/// 
/// 

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn from_file(){

    }
}

impl std::str::FromStr for TaskList {
    type Err = ();

    fn from_str(s: &str) -> Result<TaskList, ()> {
        Ok(crate::parser::task(&s.to_owned()))
    }
}