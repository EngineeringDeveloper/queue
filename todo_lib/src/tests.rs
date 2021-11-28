#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use std::{ str::FromStr};
use crate::{utils::Recurrence, Date, Task};

#[test]
fn generate_task_from_str() {
    // assert that a full string parses correctly
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
        // tags: HashMap::new(),
    };
    expected.projects.sort();
    expected.contexts.sort();
    expected.hashtags.sort();
    let input = "x (A) 2021-11-25 2021-11-23 rec:0d Task Subject #hashtag +priority +multiplePriority @context @othercont cre:ME resp:YOU due:2021-11-27";
    assert_eq!(expected, Task::from_str(input).unwrap());
    
    // assert assert edge case:
    // missing most attributes
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
        // tags: HashMap::new(),
    };
    let input = "Task with no extra description";
    assert_eq!(expected, Task::from_str(input).unwrap());

    // assert assert edge case:
    // odd orders

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
        // tags: HashMap::new(),
    };
    expected.projects.sort();
    expected.contexts.sort();
    expected.hashtags.sort();
    assert_eq!(expected, Task::from_str(input).unwrap());
}
