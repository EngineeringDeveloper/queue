
pub mod task;
pub mod task_list;
mod utils;

#[cfg(test)]
mod tests;

// Task struct is directly accessable from the library
pub use task::Task;
// So is the TaskList Struct
pub use task_list::TaskList;

// Globally use Date
use chrono::NaiveDate as Date;
