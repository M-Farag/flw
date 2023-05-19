use std::{fs::{self, File}, io::{BufRead, BufReader}};

use serde::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub data:Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaskList {
    pub tasks:Vec<Task>
}

impl TaskList {
    pub fn read_tasks(tasks_file_path:&str) -> Vec<Task>
    {
        let file_handler = File::open(tasks_file_path).unwrap();
        let file_buf_read = BufReader::new(file_handler);
        let task_list:TaskList = serde_yaml::from_reader(file_buf_read).unwrap();
        for task in &task_list.tasks {
            println!("{:?}", task);
        }
        task_list.tasks
    }
}
