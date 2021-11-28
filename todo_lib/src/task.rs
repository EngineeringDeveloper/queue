use crate::utils::*;
use crate::Date;
/// Task struct
/// a task is based off the todo.txt format
///
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// get the users username whoami::username()
use whoami;

struct Priority;

impl Priority {
    fn lowest() -> u8 {
        26
    }

    fn highest() -> u8 {
        1
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub created_by: Option<String>,
    pub assigned_to: Option<String>,
    pub create_date: Option<Date>,
    pub finish_date: Option<Date>,
    pub finished: bool,
    // pub threshold_date: Option<Date>,
    pub due_date: Option<Date>,
    pub recurrence: Option<Recurrence>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub hashtags: Vec<String>,
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
            // tags: HashMap::new(),
        }
    }

    /// Coverts a string to a task.
    pub fn parse(input_string: &str) -> Self {
        // initialise a default state
        let mut task = Task::default();
        let mut workingString = input_string.to_owned();


        // Sort out the ordered sections
        // has the task been completed
        if workingString.starts_with("x ") {
            task.finished = true;
            workingString = workingString["x ".len()..].trim().to_owned();
        }
        // if there is a priority tag
        if workingString.starts_with('(') {
            let priority = &workingString;
            match parse_priority(priority) {
                Err(_) => {
                    task.subject = workingString.to_string();
                    return task;
                }
                Ok(p) => {
                    task.priority = p;
                    workingString = workingString[priority.len()..].trim().to_owned();
                }
            }
        }

        // if there is a date next in the working string
        match try_read_date(&workingString) {
            None => {}
            Some(first_date) => {
                if task.finished {
                    // first date will be completion date if task finished
                    task.finish_date = Some(first_date);
                    workingString = workingString[10..].trim().to_owned();
                    match try_read_date(&workingString) {
                        None => {}
                        Some(second_date) => {
                            task.create_date = Some(second_date);
                            workingString = workingString[10..].trim().to_owned();
                        }
                    }
                } else {
                    // if not completed only a create date
                    task.create_date = Some(first_date);
                    workingString = workingString[10..].trim().to_owned();
                }
            }
        }
        
        // find the non ordered parts of the string
        // string is now of type "subject +projects @contexts due:date"
        // find all ther remaining task properties
        let splitter = workingString.clone();
        for word in splitter.split(' ') {
            if word.contains(PROJECT_TAG) {
                task.projects.push(word.replace(PROJECT_TAG, ""));
                // remove this string from the working String
                workingString = workingString.replace(word, "");

            } else if word.contains(CONTEXT_TAG) {
                task.contexts.push(word.replace(CONTEXT_TAG, ""));
                // remove this string from the working String
                workingString = workingString.replace(word, "");

            }else if word.contains(HASH_TAG) {
                task.hashtags.push(word.replace(HASH_TAG, ""));
                // remove this string from the working String
                workingString = workingString.replace(word, "");

            } else if word.contains(DUE_TAG_FULL) && task.due_date == None {
                if let Some(re_oc) = try_read_date(&word.replace(DUE_TAG_FULL, "")) { task.due_date = Some(re_oc) }
                // remove this string from the working String
                workingString = workingString.replace(word, "");

            }  else if word.contains(REC_TAG_FULL) {
                if let Ok(re_oc) = word.replace(REC_TAG_FULL, "").parse::<Recurrence>() { task.recurrence = Some(re_oc) }
                // remove this string from the working String
                workingString = workingString.replace(word, ""); 

            } else if word.contains(CREATOR_TAG_FULL) {
                task.created_by = Some(word.replace(CREATOR_TAG_FULL, ""));
                workingString = workingString.replace(word, "");

            } else if word.contains(ASSIGNED_TAG_FULL) {
                task.assigned_to = Some(word.replace(ASSIGNED_TAG_FULL, ""));
                workingString = workingString.replace(word, "");

            } else {
                // this word is content of the subject
            }
        }
        // the subject is the remaining text
        task.subject = workingString;

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

        if let Some(due_date) = self.due_date {
            f.write_str(format!(" due:{}", due_date.format("%Y-%m-%d")).as_str())?;
        }

        // if let Some(threshold_date) = self.threshold_date {
        //     f.write_str(format!(" t:{}", threshold_date.format("%Y-%m-%d")).as_str())?;
        // }

        // for (key, value) in &self.tags {
        //     f.write_str(format!(" {}:{}", key, value).as_str())?;
        // }

        Ok(())
    }
}
