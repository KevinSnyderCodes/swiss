use std::io::{self, BufRead};

use atty::Stream;
use clap::{Parser, Subcommand};

mod commands;

#[macro_export]
macro_rules! error_and_exit {
    ($($arg:tt)*) => {
        {
            log::error!($($arg)*);
            std::process::exit(1);
        }
    };
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Filter(commands::filter::Filter),
    Sort(commands::sort::Sort),
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
        Some(Commands::Filter(filter)) => filter.run(&mut lines),
        Some(Commands::Sort(sort)) => sort.run(&mut lines),
        None => {
            error_and_exit!("No command provided");
        }
    }

    for line in lines {
        println!("{}", line);
    }
}
