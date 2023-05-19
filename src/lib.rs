use clap::Parser;
use crate::task::TaskList;
use std::{fs, io::BufReader};

pub mod task;

#[derive(Debug,Parser)]
#[command(name = "flw", version = "0.1.0", about = "A command line tool for managing your flow.")]
pub struct Arguments {

    #[arg(long="tasksFile", short='f')]
    pub tasksFile: String,

    #[arg(long="inputFile", short='i')]
    pub inputFile: String,

    #[arg(long="safeMode", short='s')]
    pub safeMode: bool,
}

impl Arguments {
    pub fn get() -> Self {
        Arguments::parse()
    }

    pub fn run(&self) {
        let tasks = TaskList::read_tasks(&self.tasksFile);
        let input_file_handler = fs::File::open(&self.inputFile).unwrap();
        let input_file_buf_read = BufReader::new(input_file_handler);
    }
}

