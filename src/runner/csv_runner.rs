use std::fmt::Write;
use std::fs;
use std::io::{BufRead, BufReader};

use csv::{ReaderBuilder, WriterBuilder};

use crate::task::{Task, TaskList, TaskOperation, self};
use crate::runner::runner_trait::RunnerTrait;

use std::collections::HashMap;


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
           let input_file_handler = fs::OpenOptions::new().read(true).write(true).open("tmp_input.csv").unwrap();
           let  input_file_buffer = BufReader::new(input_file_handler);
           let mut reader = ReaderBuilder::new().has_headers(true).from_reader(input_file_buffer); 

            // printing headers
            let headers = reader.headers().unwrap();            

            // printing first column
            for record in reader.records() {
                let record = record.unwrap();
                println!("{}", record.get(2).unwrap());
            }
            

        }

    }



}