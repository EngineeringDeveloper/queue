use std::{collections::HashMap, str::FromStr};

use crate::{Task, Date, utils::Recurrence};

#[test]
fn generate_task_from_str() {
    let expected = Task{
        subject: String::from("Priority"),
        priority: 1,
        created_by: Some("ME".into()),
        assigned_to: Some("YOU".into()),
        create_date: Date::from_ymd_opt(2021, 11, 23),
        finish_date: Date::from_ymd_opt(2021, 11, 25),
        finished: true,
        // threshold_date: Date::from_ymd_opt(2021, 11, 26),
        due_date: Date::from_ymd_opt(2021, 11, 27),
        recurrence: Some(Recurrence::default()),
        contexts: vec!["context".into(), "othercont".into()],
        projects: vec!["priority".into(), "multiplepriority".into()],
        hashtags: vec!["hashtag".into()],
        // tags: HashMap::new(), 
    };
    let input = "x (A) 2021-11-23 2021-11-25 rec:0d Priority #hashtag +priority +multiplePriortiy @context @othercont cre:ME resp:YOU due:2021-11-27";
    assert_eq!(expected, Task::from_str(input).unwrap());
}