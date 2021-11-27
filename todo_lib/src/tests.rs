use std::str::FromStr;

use crate::{Task, Date};

#[test]
fn generate_task_from_str() {
    let expected = Task{
        subject: String::from("Priority"),
        priority: Priority::highest(),
        create_date: Date::from_ymd_opt(2021, 11, 23),
        finish_date: Date::from_ymd_opt(2021, 11, 25),
        finished: True,
        threshold_date: Date::from_ymd_opt(2021, 11, 26),
        due_date: Date::from_ymd_opt(2021, 11, 27),
        contexts: vec!["context", "othercont"],
        projects: vec!["priority", "multiplepriority"],
        hashtags: vec!["hashtag"],
        tags: BTreeMap::new(), 
    };
    let input = "x (A) 2021-11-23 Priority #hashtag +priority +multiplePriortiy @context @othercont";
    assert_eq!(expected, Task::from_str(input));
}