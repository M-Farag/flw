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
    pub fn read_tasks() 
    {
        let file_handler = File::open("tasks.yml").unwrap();
        let file_buf_read = BufReader::new(file_handler);
        let task_list:TaskList = serde_yaml::from_reader(file_buf_read).unwrap();

        for task in task_list.tasks {
            println!("{:?}", task);
        }
    }
}
