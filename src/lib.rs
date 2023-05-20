use clap::Parser;
use crate::task::TaskList;
use std::{fs, io::BufReader};

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
        let input_file_handler = fs::File::open(&self.input_file).unwrap();
        let input_file_buf_read = BufReader::new(input_file_handler);

        tasks.iter().for_each(
            |task| {
                println!("{:?}", task);
            }
        );
    }
}

