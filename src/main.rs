use std::io::{self, Write};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter)]
enum Command {
    Help,
    Quit,
}

#[derive(Debug)]
enum CommandError {
    EmptyInput,
    UnknownCommand(String),
    #[allow(dead_code)]
    MissingArgument(String),
}

fn print_help() {
    println!("Available commands:");
    for cmd in Command::iter() {
        println!("- {}", cmd.to_string().to_lowercase().trim());
    }
    println!();
}

fn parse_command(input: &str) -> Result<Command, CommandError> {
    let mut parts = input.trim().split_whitespace();

    let cmd_str = parts.next().ok_or(CommandError::EmptyInput)?;

    match cmd_str {
        "help" => Ok(Command::Help),
        "quit" => Ok(Command::Quit),
        _ => Err(CommandError::UnknownCommand(cmd_str.to_string())),
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            println!("\nGoodbye!");
            break;
        }

        match parse_command(input.trim()) {
            Ok(Command::Help) => print_help(),
            Ok(Command::Quit) => break,

            Err(CommandError::EmptyInput) => continue,
            Err(CommandError::UnknownCommand(cmd)) => {
                eprintln!("Unknown command: '{}'. Type 'help'.", cmd);
            }
            Err(CommandError::MissingArgument(msg)) => {
                eprintln!("Missing argument: '{}'", msg);
            }
        }
    }
    std::process::exit(0);
}
