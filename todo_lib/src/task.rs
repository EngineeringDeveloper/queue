use crate::utils::*;
use crate::Date;
/// Task struct
/// a task is based off the todo.txt format
///
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
struct Priority;

impl Priority {
    fn lowest() -> u8 {
        26
    }

    // fn highest() -> u8 {
    //     1
    // }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Task {
    pub finished: bool,
    pub priority: u8,
    pub finish_date: Option<Date>,
    pub create_date: Option<Date>,
    pub subject: String,
    pub created_by: Option<String>,
    pub assigned_to: Option<String>,
    // pub threshold_date: Option<Date>,
    pub due_date: Option<Date>,
    pub recurrence: Option<Recurrence>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub hashtags: Vec<String>,
    pub input_hash: Option<u64>,
    // pub tags: HashMap<String, String>,
}

impl Task {
    pub fn default() -> Self {
        Self {
            subject: String::new(),
            priority: Priority::lowest(),
            created_by: None,
            assigned_to: None,
            create_date: None,
            finish_date: None,
            finished: false,
            // threshold_date: None,
            due_date: None,
            recurrence: None,
            contexts: Vec::new(),
            projects: Vec::new(),
            hashtags: Vec::new(),
            input_hash: None
            // tags: HashMap::new(),
        }
    }

    /// Coverts a string to a task.
    pub fn parse(input_string: &str) -> Self {
        // initialise a default state
        let mut task = Task::default();
        // Create the hash of the input
        let mut def_hasher = DefaultHasher::new();
        input_string.hash(&mut def_hasher);
        task.input_hash = Some(def_hasher.finish());

        let mut working_string = input_string.to_owned();

        // Sort out the ordered sections
        // has the task been completed
        if working_string.starts_with("x ") {
            task.finished = true;
            working_string = working_string["x ".len()..].trim().to_owned();
        }

        // if there is a priority tag
        if working_string.starts_with('(') {
            let priority = working_string.chars().collect::<Vec<char>>()[..3]
                .iter()
                .collect::<String>();
            match parse_priority(&priority) {
                Err(_) => {
                    task.priority = NO_PRIORITY;
                }
                Ok(p) => {
                    task.priority = p;
                    working_string = working_string[priority.len()..].trim().to_owned();
                }
            }
        }

        // if there is a date next in the working string
        match try_read_date(&working_string) {
            None => {}
            Some(first_date) => {
                if task.finished {
                    // first date will be completion date if task finished
                    task.finish_date = Some(first_date);
                    working_string = working_string[10..].trim().to_owned();
                    match try_read_date(&working_string) {
                        None => {}
                        Some(second_date) => {
                            task.create_date = Some(second_date);
                            working_string = working_string[10..].trim().to_owned();
                        }
                    }
                } else {
                    // if not completed only a create date
                    task.create_date = Some(first_date);
                    working_string = working_string[10..].trim().to_owned();
                }
            }
        }

        // find the non ordered parts of the string
        // string is now of type "subject +projects @contexts due:date"
        // find all ther remaining task properties
        let splitter = working_string.clone();
        for word in splitter.split(' ') {
            if word.contains(PROJECT_TAG) {
                task.projects.push(word.replace(PROJECT_TAG, ""));
                // remove this string from the working String
                working_string = working_string.replace(word, "");
            } else if word.contains(CONTEXT_TAG) {
                task.contexts.push(word.replace(CONTEXT_TAG, ""));
                // remove this string from the working String
                working_string = working_string.replace(word, "");
            } else if word.contains(HASH_TAG) {
                task.hashtags.push(word.replace(HASH_TAG, ""));
                // remove this string from the working String
                working_string = working_string.replace(word, "");
            } else if word.contains(DUE_TAG_FULL) && task.due_date == None {
                if let Some(re_oc) = try_read_date(&word.replace(DUE_TAG_FULL, "")) {
                    task.due_date = Some(re_oc)
                }
                // remove this string from the working String
                working_string = working_string.replace(word, "");
            } else if word.contains(REC_TAG_FULL) {
                if let Ok(re_oc) = word.replace(REC_TAG_FULL, "").parse::<Recurrence>() {
                    task.recurrence = Some(re_oc)
                }
                // remove this string from the working String
                working_string = working_string.replace(word, "");
            } else if word.contains(CREATOR_TAG_FULL) {
                task.created_by = Some(word.replace(CREATOR_TAG_FULL, ""));
                working_string = working_string.replace(word, "");
            } else if word.contains(ASSIGNED_TAG_FULL) {
                task.assigned_to = Some(word.replace(ASSIGNED_TAG_FULL, ""));
                working_string = working_string.replace(word, "");
            } else {
                // this word is content of the subject
            }
        }
        // sort all the vectors
        task.projects.sort();
        task.contexts.sort();
        task.hashtags.sort();

        // the subject is the remaining text
        task.subject = working_string.trim().to_owned();

        task
    }

    pub fn complete(&mut self) {
        let today = chrono::Local::now().date().naive_local();

        self.finished = true;
        if self.create_date.is_some() {
            self.finish_date = Some(today);
        }
    }

    pub fn uncomplete(&mut self) {
        self.finished = false;
        self.finish_date = None;
    }
}

impl std::str::FromStr for Task {
    type Err = ();

    fn from_str(s: &str) -> Result<Task, ()> {
        Ok(Task::parse(&s.to_owned()))
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.finished {
            f.write_str("x ")?;
        }

        if self.priority < Priority::lowest() {
            let priority = (b'A' + self.priority) as char;

            f.write_str(format!("({}) ", priority).as_str())?;
        }

        if let Some(finish_date) = self.finish_date {
            f.write_str(format!("{} ", finish_date.format("%Y-%m-%d")).as_str())?;
        }

        if let Some(create_date) = self.create_date {
            f.write_str(format!("{} ", create_date.format("%Y-%m-%d")).as_str())?;
        }

        f.write_str(self.subject.as_str())?;
        
        if let Some(created_by) = self.created_by {
            f.write_str(format!(" {}{}",CREATOR_TAG_FULL , created_by).as_str())?;
        }
        
        if let Some(assigned_to) = self.assigned_to {
            f.write_str(format!(" {}{}",ASSIGNED_TAG_FULL , assigned_to).as_str())?;
        }

        if let Some(due_date) = self.due_date {
            f.write_str(format!(" {}{}",DUE_TAG_FULL , due_date.format("%Y-%m-%d")).as_str())?;
        }

        if let Some(recurrence) = self.recurrence {
            f.write_str(format!(" {}{}",REC_TAG_FULL , recurrence).as_str())?;
        }

        for context in self.contexts {
            f.write_str(format!(" {}{}",CONTEXT_TAG , context).as_str())?;
        }
        
        for projects in self.projects {
            f.write_str(format!(" {}{}",PROJECT_TAG , projects).as_str())?;
        }
        
        for hashtags in self.hashtags {
            f.write_str(format!(" {}{}",HASH_TAG , hashtags).as_str())?;
        }

        Ok(())
    }
}
