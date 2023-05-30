use std::fs;
use std::io::{BufReader, BufWriter};
use regex::Regex;
use csv::{ReaderBuilder, WriterBuilder};

use crate::task::{Task, TaskList, TaskOperation};
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
           let input_file_handler = fs::OpenOptions::new().read(true).open("tmp_input.csv").unwrap();
           let  input_file_buffer = BufReader::new(input_file_handler);
           let mut reader = ReaderBuilder::new().has_headers(true).from_reader(input_file_buffer); 
           
           let output_file_handler = fs::OpenOptions::new().write(true).create(true).append(true).open("tmp_output.csv").unwrap();
           let output_file_buffer = BufWriter::new(output_file_handler);
           let mut writer = WriterBuilder::new().has_headers(true).from_writer(output_file_buffer);

            // printing headers
            let headers = reader.headers().unwrap();            
            // ToDo
            // Map the headers & indexes to a hashmap
            // Dynamically get the index
            
            // printing first column
            for record in reader.records() {
                let mut record = record.unwrap();
                // Modify record
                let modified_record:Vec<String> = record.iter().enumerate().map(
                    |(i,field)| {
                        
                        if i == 1 {
                            let pattern = Regex::new(&task.data[1]).unwrap();
                            let field_replaced = pattern.replace_all(&field, &task.data[2]);
                            return format!("{}", field_replaced);
                        }
                        field.to_string()
                    }
                ).collect();

                writer.write_record(&modified_record).unwrap();
            }
            writer.flush().unwrap();
            fs::rename("tmp_output.csv", "tmp_input.csv").unwrap();
       }
    }



}