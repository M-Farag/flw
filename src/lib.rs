use clap::Parser;
use crate::runner::text_runner::TextRunner;
use crate::runner::runner_trait::RunnerTrait;

mod runner;
mod task;

#[derive(Debug,Parser)]
#[command(name = "flw", version = "0.1.0", about = "A command line tool for managing your flow.")]
struct Arguments {

    #[arg(long, short='f')]
    tasks_file: String,

    #[arg(long, short='i')]
    input_file: String,

    #[arg(long, short='t', default_value = "txt")]
    process_input_file_as: String
}

pub struct FLW {
    arguments: Arguments
}

impl FLW {
    pub fn new() -> Self
    {
        let arguments = Arguments::parse();
        FLW {
            arguments
        }
    }
    pub fn run(&self)
    {
        match self.arguments.process_input_file_as.as_str() {
            "txt" => {
                let text_runner = TextRunner::new(self.arguments.tasks_file.clone(), self.arguments.input_file.clone());
                text_runner.run();
            },
            _ => println!("Unsupported file type")
        }
        
    }
}



