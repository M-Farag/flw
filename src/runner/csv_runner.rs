use std::fs;
use std::io::{BufReader, BufWriter};
use regex::Regex;
use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashMap;
use crate::task::{Task, TaskList, TaskOperation};
use crate::runner::runner_trait::RunnerTrait;
use std::time::Instant;



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
                        println!("Starting task {:?}", task);
                        let time_start = Instant::now();
                        process_replace_task(task);
                        println!("Task {:?} completed in {:?}", task, time_start.elapsed());

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

            // Processing headers
            let headers = reader.headers().unwrap();
            let mut headers_indexes:HashMap<String,usize> = HashMap::new();
            for (i,header) in headers.iter().enumerate() {
                headers_indexes.insert(header.to_string(),i);
            }

            
            // Write headers to the output file
            writer.write_record(reader.headers().unwrap()).unwrap();
            writer.flush().unwrap();

            let requested_column_index = *headers_indexes.get(&task.data[0]).unwrap();            
            for record in reader.records() {
                let record = record.unwrap();
                // Modify record
                let modified_record:Vec<String> = record.iter().enumerate().map(
                    |(i,field)| {
                        if i == requested_column_index && field == &task.data[1] {
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