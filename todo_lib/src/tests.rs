#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use crate::{utils::Recurrence, Date, Task, TaskList};
use std::hash::{Hash, Hasher};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    str::FromStr,
};

#[test]
fn test_generate_task_from_str() {
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
fn test_generate_task_list_from_path() {
    let path = String::from("../testFiles/todo.txt");
    TaskList::from_file(&path).unwrap();
}

#[test]
fn test_add_task() {
    let mut task_list = TaskList::default();
    task_list.add_task(Task::default());
    
    let mut hash = HashMap::new();
    hash.insert(26, vec![Task::default()]);
    let expected_task_list =
        TaskList::new("".into(),
         vec![Task::default()],
          hash);
    
    assert_eq!(task_list, expected_task_list);
}

#[test]
fn test_modify_task() {

    let mut task_list = TaskList::from_str("Test Task\nOtherTask").unwrap();
    
    let mut new_task = Task::from_str("Test Task").unwrap(); // to have the same index_hash
    new_task.subject = "New_task".into();
    task_list.change_task(new_task);
    
    assert!(task_list.tasks.iter().find(|task| {task.subject == *"New_task"}) != None);
}

#[test]
fn test_save_task_list() {
    let task_list = TaskList::from_file(&"../testFiles/todo.txt".to_owned()).unwrap();
    println!("{}", task_list);
    task_list.save().unwrap();
}
