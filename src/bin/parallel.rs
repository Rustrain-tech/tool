use clap::Parser;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tool::exec;

#[derive(Parser)]
struct Cli {
    /// command
    command: String,
    /// path to in folder
    #[clap(short = 'd', long = "dir", default_value = "in")]
    input: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    for file in std::fs::read_dir(&cli.input).unwrap() {
        let file = file.unwrap();
        let filename = file.file_name();
        let path = file.path();
        let mut p = Command::new(cli.command.as_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        match exec(&mut p, &path) {
            Ok(score) => println!("Case {:?} Score = {}", filename, score),
            Err(e) => eprintln!("{}", e),
        }
    }
}
