use colored::*;
use std::fmt::Display;

pub fn success<T: Display>(msg: T) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn error<T: Display>(msg: T) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

pub fn info<T: Display>(msg: T) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

pub fn warn<T: Display>(msg: T) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

pub fn streaming_output(chunk: &str) {
    print!("{}", chunk);
    std::io::stdout().flush().unwrap();
}