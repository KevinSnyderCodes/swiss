use std::io::{self, BufRead};
use std::process::exit;

use atty::Stream;
use clap::{Parser, Subcommand};
use log::error;

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

macro_rules! error_and_exit {
    ($($arg:tt)*) => {
        error!($($arg)*);
        exit(1);
    };
}

fn main() {
    let cli = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .verbosity(stderrlog::LogLevelNum::Info)
        .init()
        .unwrap();

    if atty::is(Stream::Stdin) {
        error_and_exit!("No input provided");
    }

    let mut lines = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    match cli.command {
        Some(Commands::Sort(sort)) => sort.run(&mut lines),
        None => {
            error_and_exit!("No command provided");
        }
    }

    for line in lines {
        println!("{}", line);
    }
}
