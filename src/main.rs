mod cli;
mod counter;

use clap::Parser;
use cli::Args;
use counter::Counter;
use std::fs::File;
use std::io::{self, IsTerminal};
use std::process;

fn print_counts(counter: &Counter, args: &Args, label: &str) {
    if args.bytes {
        println!("{label}: Byte count: {}", counter.bytes);
    }
    if args.lines {
        println!("{label}: Line count: {}", counter.lines);
    }
    if args.words {
        println!("{label}: Word count: {}", counter.words);
    }
    if args.chars {
        println!("{label}: Char count: {}", counter.chars);
    }
}

fn main() {
    let mut args = Args::parse();

    if !args.bytes && !args.lines && !args.words && !args.chars {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    let stdin = io::stdin();
    if !stdin.is_terminal() {
        let counter = Counter::from_reader(stdin.lock()).unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {e}");
            process::exit(1);
        });
        print_counts(&counter, &args, "stdin");
    } else if !args.files.is_empty() {
        for path in &args.files {
            let file = File::open(path).unwrap_or_else(|e| {
                eprintln!("Error reading '{}': {e}", path.display());
                process::exit(1);
            });
            let counter = Counter::from_reader(file).unwrap_or_else(|e| {
                eprintln!("Error reading '{}': {e}", path.display());
                process::exit(1);
            });
            print_counts(&counter, &args, &path.display().to_string());
        }
    } else {
        eprintln!("Error: provide at least one file, or pipe data via stdin.");
        process::exit(1);
    }
}
