/// Task struct
/// a task is based off the todo.txt format
/// 

use serde::{Serialize, Deserialize};
use crate::Date;

struct Priority;

impl Priority {
    fn lowest() -> u8 {
        26
    }

    fn highest() -> u8 {
        1
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub create_date: Option<Date>,
    pub finish_date: Option<Date>,
    pub finished: bool,
    pub threshold_date: Option<Date>,
    pub due_date: Option<Date>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub hashtags: Vec<String>,
    pub tags: BTreeMap<String, String>,
}

impl Task {

    pub fn default() -> Self {
        Self {
            subject: String::new(),
            priority: Priority::lowest(),
            create_date: None,
            finish_date: None,
            finished: false,
            threshold_date: None,
            due_date: None,
            contexts: Vec::new(),
            projects: Vec::new(),
            hashtags: Vec::new(),
            tags: BTreeMap::new(),
        }
    }

    fn parse_special_tags(&mut self, base: Date) {
        let mut old_tags: Vec<String> = Vec::new();
        let mut new_tags: Vec<String> = Vec::new();
        for (name, value) in &self.tags {
            if name == "rec" {
                if let Ok(rec) = value.parse::<utils::Recurrence>() {
                    self.recurrence = Some(rec);
                }
            }
            if name == "t" {
                if let Ok(dt) = utils::parse_date(value, base) {
                    self.threshold_date = Some(dt);
                    let old_tag = format!("{}:{}", name, value);
                    let new_tag = format!("{}:{}", name, utils::format_date(dt));
                    if old_tag != new_tag {
                        old_tags.push(old_tag);
                        new_tags.push(new_tag);
                    }
                }
            }
            if name == "due" {
                if let Ok(dt) = utils::parse_date(value, base) {
                    self.due_date = Some(dt);
                    let old_tag = format!("{}:{}", name, value);
                    let new_tag = format!("{}:{}", name, utils::format_date(dt));
                    if old_tag != new_tag {
                        old_tags.push(old_tag);
                        new_tags.push(new_tag);
                    }
                }
            }
        }

        for (old, new) in old_tags.iter().zip(new_tags.iter()) {
            self.replace_tag(old, new);
            println!("after replacing [{}] with [{}]: [{}]", old, new, self.subject);
        }
    }

    /// Coverts a string to a task.
    pub fn parse(s: &str) -> Self {
        let mut task = Task::validate(s);
        task.parse_special_tags();
        task
    }

    fn validate(inputString: &str) -> Self {
        // initialise a default state
        let mut task = Task {
            subject: String::new(),
            priority: Priority::lowest(),
            create_date: None,
            finish_date: None,
            finished: false,
            threshold_date: None,
            due_date: None,
            contexts: vec![],
            projects: vec![],
            // recurrence: None,
            hashtags: vec![],
            tags: vec![],
        };
        let mut workingString = inputString;

        // has the task been completed
        if workingString.starts_with("x ") {
            task.finished = true;
            workingString = workingString["x ".len()..].trim();
        }
        // if there is a priority tag
        if workingString.starts_with('(') {
            let priority = workingString;
            match parse_priority(priority) {
                Err(_) => {
                    task.subject = workingString.to_string();
                    return task;
                }
                Ok(p) => {
                    task.priority = p;
                    workingString = workingString[priority.len()..].trim();
                }
            }
        }

        // if there is a 
        match try_read_date(workingString) {
            None => {
                task.subject = workingString.to_string();
                return task;
            }
            Some(dt) => {
                if task.finished {
                    task.finish_date = Some(dt);
                } else {
                    task.create_date = Some(dt);
                }
                match workingString.find(' ') {
                    None => return task,
                    Some(idx) => workingString = workingString[idx + 1..].trim(),
                }
                if !task.finished {
                    task.subject = workingString.to_string();
                    return task;
                }
            }
        }
        match try_read_date(workingString) {
            None => task.subject = workingString.to_string(),
            Some(dt) => {
                task.create_date = Some(dt);
                if let Some(idx) = workingString.find(' ') {
                    task.subject = workingString[idx + 1..].trim().to_string();
                }
            }
        }
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
        Ok(crate::parser::task(&s.to_owned()))
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

        if let Some(threshold_date) = self.threshold_date {
            f.write_str(format!(" t:{}", threshold_date.format("%Y-%m-%d")).as_str())?;
        }

        for (key, value) in &self.tags {
            f.write_str(format!(" {}:{}", key, value).as_str())?;
        }

        Ok(())
    }
}

fn next_word(s: &str) -> &str {
    if s.is_empty() {
        return s;
    }
    match s.find(' ') {
        None => s,
        Some(p) => &s[..p],
    }
}


/// Converts a string like "(A)" to a u8 up to 26
pub fn parse_priority(s: &str) -> Result<u8, String> {

    if s.len() != 3 {
        return Err(format!("invalid priority '{}'", s));
    }
    // removes all the characters not below
    let trimmed = s.trim_matches(|c| c == ' ' || c == '(' || c == ')');
    if trimmed.len() != 1 {
        return Err(format!("invalid priority '{}'", s));
    }

    let priority = trimmed.bytes().next().expect("impossible");
    // if not a capital letter
    if !(b'A'..=b'Z').contains(&priority) {
        return Err(format!("invalid priority '{}'", s));
    }
    // adjusted byte as u8
    Ok((priority - b'A') as u8)
}

fn try_read_date(input: &str) -> Option<Date> {
    // if there are 2 dashes in the 10 char length then we espect a date
    let target = input.chars()[..11];
    if target.matches("-").count() == 2 {
        let mut values = vec![];
        for section in target.collect().split("-") {
            match section.parse::<u32>() {
                Err(_) => return None,
                Ok(value) => values.push()
            }
        }
        // Check for other not valid dates
        if values.len() != 3 {
            return Err(format!("invalid date '{}'", input));
        }
        if values[0] == 0 {
            return Err(format!("invalid year '{}'", input));
        }
        if values[1] == 0 || values[1] > 12 {
            return Err(format!("invalid month '{}'", input));
        }
        if values[2] == 0 || values[2] > 31 {
            return Err(format!("invalid day '{}'", input));
        }
        Ok(Date::from_ymd(values[0] as i32, values[1], values[2]))
        
    } else {
        None
    }
}