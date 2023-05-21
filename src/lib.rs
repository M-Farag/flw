use clap::Parser;
use crate::task::{TaskOperation, TaskList, Task};
use std::{fs, io::{BufReader, BufWriter, BufRead}};
use regex::Regex;
use std::io::Write;

pub mod task;

#[derive(Debug,Parser)]
#[command(name = "flw", version = "0.1.0", about = "A command line tool for managing your flow.")]
struct Arguments {

    #[arg(long, short='f')]
    tasks_file: String,

    #[arg(long, short='i')]
    input_file: String,

    #[arg(long, short='s')]
    safe_mode: bool,
}

pub struct Runner {
    tasks_file: String,
    input_file: String,
    safe_mode: bool,
}


impl Runner {

    pub fn new() -> Self
    {
        let args = Arguments::parse();
        Runner {
            tasks_file: args.tasks_file,
            input_file: args.input_file,
            safe_mode: args.safe_mode,
        }
    }

    pub fn run(&self) {
        let tasks = TaskList::read_tasks(&self.tasks_file);
        
        // Copy the input file to a temporary file that will be used for the operations
        fs::copy(&self.input_file, "tmp_input.txt").unwrap();
        
        tasks.iter().for_each(
            |task| {
                if let Some(task_operation) = task.operation.as_ref() {
                    match task_operation {
                        TaskOperation::Replace => {
                            process_replace_task(task);
                        },
                        TaskOperation::Count => {
                            println!("Count in task: {:#?}", task);
                        }
                    }
                }
            }
        );

        fn process_replace_task(task: &Task) {
            let input_file = fs::File::open("tmp_input.txt").unwrap();
            let input_file_buf_read = BufReader::new(input_file);
            
            let output_file = fs::File::create("tmp_output.txt").unwrap();
            let mut output_file_buf_write = BufWriter::new(output_file);

            let pattern = Regex::new(&task.data[0]).unwrap();
            for line in input_file_buf_read.lines() {
                let line = line.unwrap();
                let line_replaced = pattern.replace_all(&line, &task.data[1]);
                writeln!(output_file_buf_write,"{}",line_replaced).unwrap();
            }

            fs::rename("tmp_output.txt", "tmp_input.txt").unwrap();
        }
    }
}

