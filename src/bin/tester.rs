use clap::Parser;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tool::exec;

#[derive(Parser)]
struct Cli {
    /// command
    command: String,
    /// path to input file
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut p = Command::new(cli.command.as_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    match exec(&mut p, &cli.input) {
        Ok(score) => println!("Score = {}", score),
        Err(e) => eprintln!("{}", e),
    }
}
