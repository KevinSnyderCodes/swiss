use std::io::{self, BufRead};

use atty::Stream;
use clap::{Parser, Subcommand};
use core::panic;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Sort(commands::sort::Sort),
}

fn main() {
    if atty::is(Stream::Stdin) {
        panic!("No input provided");
    }

    let mut lines = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Sort(sort)) => sort.run(&mut lines),
        None => {
            panic!("No command provided");
        }
    }

    for line in lines {
        println!("{}", line);
    }
}
