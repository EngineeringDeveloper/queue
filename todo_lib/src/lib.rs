
mod task;
mod taskList;
mod utils;

#[cfg(test)]
mod tests;

// Task struct is directly accessable from the library
use task::Task;
// So is the TaskList Struct
use taskList::TaskList;

// Globally use Date
use chrono::NaiveDate as Date;
