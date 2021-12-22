use std::{
    collections::HashMap,
    convert::From,
    fs,
    io::{self, Read, Write},
    path::Path,
    str::FromStr
};

use crate::Task;
use serde::{Deserialize, Serialize};
use serde_json::error;

/// Task list struct is a container for a vec of tasks
/// It also does operations to keep track of changes sort and filter
///

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub source: String,
    pub tasks: Vec<Task>,
    pub prioritised_tasks: HashMap<u8, Vec<Task>>,
}

impl TaskList {
    pub fn new(source: String, tasks: Vec<Task>, prioritised_tasks: HashMap<u8, Vec<Task>>) -> Self { Self { source, tasks, prioritised_tasks } }

    pub fn default() -> Self {
        Self {
            source: "".into(),
            tasks: vec![],
            prioritised_tasks: HashMap::new(),
        }
    }

    pub fn from_file<'a, P: AsRef<Path> + std::fmt::Debug>(
        path: &'a P,
    ) -> Result<Self, io::Error>
    where
        std::string::String: From<&'a P>,
    {
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
        let tasks = vec_tasks_from_str(&contents);
        for tasklst in prioritised_tasks_from_vec_tasks(&tasks).into_values() {
            for ptask in tasklst {
                let mut found = false;
                for task in tasks.iter() {
                    if task.input_hash.unwrap() == ptask.input_hash.unwrap() {
                        println!(" {} \t Match", ptask.input_hash.unwrap());
                        found = true;
                        break
                    }
                }
                if !found {
                    println!("{} \t No Match ", ptask.input_hash.unwrap());
                }

            }
        }
        let prioritised_tasks = prioritised_tasks_from_vec_tasks(&tasks);

        Ok(TaskList {
            source: String::from(path),
            tasks,
            prioritised_tasks,
        })
        // Ok(TaskList::from_str(&contents).expect("from Str should always return Ok"))
    }

    pub fn change_task(&mut self, new_task: Task) {
        // takes in the new task and uses its identifier to change the stored value
        // change the task in the taskvec and hash
        // TODO: The hash is not being identified correctly
        println!("new Task {}", new_task.input_hash.expect("have to have a hash"));
        for task in self.tasks.iter() {
            println!("ext Task {}", task.input_hash.expect("have to have a hash"))
        }
        // self.tasks.iter().map(|x| println!("All tasks{}", x.input_hash.expect("have to have a hash")));
        if let Some(task_index) = self.tasks.iter().position(|val| {val.input_hash == new_task.input_hash}) {
            println!("found");
            self.tasks.remove(task_index);
            self.tasks.push(new_task);
        } else {
            println!("not found");
            self.add_task(new_task);
            // not sure this is the desired behavior
            // when the task is not already in the list it is added
        }
        // is this the best way to correct the hashmap, 
        // this creates from new over -- find, remove + move
        self.prioritised_tasks = prioritised_tasks_from_vec_tasks(&self.tasks)

    }

    pub fn add_task(&mut self, new_task: Task) {
        self.tasks.push(new_task);
        self.prioritised_tasks = prioritised_tasks_from_vec_tasks(&self.tasks)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        // saves to the contained self.source
        let path = self.source.clone();
        let mut todo_file = fs::File::create(&path)?;
        todo_file.write_all(format!("{}", self).as_bytes())
    }
}

impl std::str::FromStr for TaskList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let tasks = vec_tasks_from_str(s);
        Ok(TaskList {
            source: "".into(),
            prioritised_tasks: prioritised_tasks_from_vec_tasks(&tasks),
            tasks,
        })
    }
}

impl std::fmt::Display for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        for task in &self.tasks {
            f.write_str(format!("{}\n", task).as_str())?
        }
        Ok(())
    }
}

fn vec_tasks_from_str(s: &str) -> Vec<Task> {
    s.lines()
        .filter_map(|line| Task::from_str(line).ok())
        .collect()
}

fn prioritised_tasks_from_vec_tasks(vt: &[Task]) -> HashMap<u8, Vec<Task>> {
    let mut hashmap = HashMap::new();
    for task in vt {
        hashmap
                .entry(task.priority)
                .or_insert_with(Vec::new)
                .push(task.clone());
    }
    hashmap
}
