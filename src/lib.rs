use clap::Parser;
use crate::task::TaskList;

pub mod task;

#[derive(Debug,Parser)]
#[command(name = "flw", version = "0.1.0", about = "A command line tool for managing your flow.")]
pub struct Arguments {

#[arg(short='c', long)]
    pub command: String,
}

impl Arguments {
    pub fn get() -> Self {
        Arguments::parse()
    }

    pub fn run_command(&self) {
        match self.command.as_str() {
            "tasks" => TaskList::read_tasks(),
            _ => println!("No command found"),
        }
    }
}

