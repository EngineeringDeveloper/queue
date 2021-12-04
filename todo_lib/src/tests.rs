#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use std::{ str::FromStr, collections::hash_map::DefaultHasher};
use crate::{utils::Recurrence, Date, Task, TaskList};
use std::hash::{Hash, Hasher};


#[test]
fn generate_task_from_str() {
    // assert that a full string parses correctly
    let input = "x (A) 2021-11-25 2021-11-23 rec:0d Task Subject #hashtag +priority +multiplePriority @context @othercont cre:ME resp:YOU due:2021-11-27";
    let mut def_hasher = DefaultHasher::new();
    input.to_owned().hash(&mut def_hasher);
    let mut expected = Task {
        subject: String::from("Task Subject"),
        priority: 0,
        created_by: Some("ME".into()),
        assigned_to: Some("YOU".into()),
        create_date: Date::from_ymd_opt(2021, 11, 23),
        finish_date: Date::from_ymd_opt(2021, 11, 25),
        finished: true,
        // threshold_date: Date::from_ymd_opt(2021, 11, 26),
        due_date: Date::from_ymd_opt(2021, 11, 27),
        recurrence: Some(Recurrence::default()),
        contexts: vec!["context".into(), "othercont".into()],
        projects: vec!["priority".into(), "multiplePriority".into()],
        hashtags: vec!["hashtag".into()],
        input_hash: Some(def_hasher.finish()),
        // tags: HashMap::new(),
    };
    expected.projects.sort();
    expected.contexts.sort();
    expected.hashtags.sort();
    assert_eq!(expected, Task::from_str(input).unwrap());
    
    // assert assert edge case:
    // missing most attributes
    let input = "Task with no extra description";
    let mut def_hasher = DefaultHasher::new();
    input.to_owned().hash(&mut def_hasher);
    let expected = Task {
        subject: String::from("Task with no extra description"),
        priority: 26,
        created_by: None,
        assigned_to: None,
        create_date: None,
        finish_date: None,
        finished: false,
        // threshold_date: Date::from_ymd_opt(2021, 11, 26),
        due_date: None,
        recurrence: None,
        contexts: vec![],
        projects: vec![],
        hashtags: vec![],
        input_hash: Some(def_hasher.finish()),
        // tags: HashMap::new(),
    };
    assert_eq!(expected, Task::from_str(input).unwrap());

    // assert assert edge case:
    // odd orders
    let input = "x (A) 2021-11-25 2021-11-23 +multiplePriority rec:0d @context resp:YOU Task Subject #hashtag +priority @othercont cre:ME due:2021-11-27";
    let mut def_hasher = DefaultHasher::new();
    input.to_owned().hash(&mut def_hasher);
    let mut expected = Task {
        subject: String::from("Task Subject"),
        priority: 0,
        created_by: Some("ME".into()),
        assigned_to: Some("YOU".into()),
        create_date: Date::from_ymd_opt(2021, 11, 23),
        finish_date: Date::from_ymd_opt(2021, 11, 25),
        finished: true,
        // threshold_date: Date::from_ymd_opt(2021, 11, 26),
        due_date: Date::from_ymd_opt(2021, 11, 27),
        recurrence: Some(Recurrence::default()),
        contexts: vec!["context".into(), "othercont".into()],
        projects: vec!["multiplePriority".into(), "priority".into()],
        hashtags: vec!["hashtag".into()],
        input_hash: Some(def_hasher.finish()),
        // tags: HashMap::new(),
    };
    expected.projects.sort();
    expected.contexts.sort();
    expected.hashtags.sort();
    assert_eq!(expected, Task::from_str(input).unwrap());
}


#[test]
fn generate_task_list_from_path() {
    let path = String::from("../testFiles/todo.txt");
    TaskList::from_file(&path).unwrap();
}