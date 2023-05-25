use std::fs;

use crate::task::{Task, TaskList, TaskOperation, self};
use crate::runner::runner_trait::RunnerTrait;


pub struct CsvRunner {
    tasks_file: String,
    input_file: String,
}

impl RunnerTrait for CsvRunner {
    fn new(tasks_file:String, input_file:String) -> Self {
        CsvRunner {
            tasks_file,
            input_file,
        }  
    }

    fn run(&self) {
        let tasks = TaskList::read_tasks(&self.tasks_file);
        fs::copy(&self.input_file, "tmp_input.csv").unwrap();
        tasks.iter().for_each(|task|{
            if let Some(task_operation) = task.operation.as_ref() {
                match task_operation {
                    TaskOperation::Replace => {
                        println!("Replacing...");
                    },
                    TaskOperation::Count => {
                        println!("Counting...");
                    }
                }
            }
        });

    }

}