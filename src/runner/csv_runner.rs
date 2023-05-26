use std::fmt::Write;
use std::fs;

use csv::{ReaderBuilder, WriterBuilder};

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
                        process_replace_task(task);
                    },            
                    _ => {
                        println!("Not implemented yet");
                    }
                }
            }
        });

        fn process_replace_task(task: &Task) {
            let input_file = fs::OpenOptions::new().read(true).open("tmp_input.csv").unwrap();

            let mut reader = ReaderBuilder::new().has_headers(true).from_reader(input_file);

            let output_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("tmp_output.csv")
                .unwrap();

            let mut writer = WriterBuilder::new().has_headers(true).from_writer(output_file);
            let task_data:&Vec<&str> = &task.data[0].split(',').collect();
            
        }

    }



}