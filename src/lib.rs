use clap::Parser;

#[derive(Debug,Parser)]
#[command(name = "flw", version = "0.1.0", about = "A command line tool for managing your flow.")]
pub struct Arguments {

#[arg(short='c', long)]
    pub command: String,
}

impl Arguments {
    pub fn run() -> Self {
        Arguments::parse()
    }
}

