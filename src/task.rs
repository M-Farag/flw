use std::{fs:: File, io::BufReader};

use serde::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub data:Vec<String>,
    
    #[serde(skip)]
    pub operation:Option<TaskOperation>,
}

impl Task {
   pub fn update_task(&mut self,data:Vec<String>, task_operation:TaskOperation)
   {
       self.operation = Some(task_operation);
       self.data = data;
   }
}

#[derive(Debug, Deserialize)]
pub struct TaskList {
    pub tasks:Vec<Task>
}

#[derive(Debug)]
pub enum TaskOperation {
    Replace,
    Count
}

impl TaskList {
    pub fn read_tasks(tasks_file_path:&str) -> Vec<Task>
    {
        let file_handler = File::open(tasks_file_path).unwrap();
        let file_buf_read = BufReader::new(file_handler);
        let mut task_list:TaskList = serde_yaml::from_reader(file_buf_read).unwrap();
         
         task_list.tasks.iter_mut().for_each(|t:&mut Task|{
            if let Some(task_operation) = TaskList::identify_task_operation(&t.data[0]) {
                let data = t.data[1..].to_vec();
                t.update_task(data, task_operation);
            }
        });
        task_list.tasks
    }

    fn identify_task_operation(task_operation:&str) -> Option<TaskOperation>
    {
        match task_operation {
            "replace" => Some(TaskOperation::Replace),
            "count" => Some(TaskOperation::Count),
            _ => None
        }
    }
}
