//! Main loop of the program.

use crate::commands::Command;
use crate::errors::CommandError;
use std::io::{self, Write};
use strum::IntoEnumIterator;

pub fn main_loop() {
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
                CommandError::UnknownCommand(cmd).handle_error();
            }
            Err(CommandError::MissingArgument(msg)) => {
                CommandError::MissingArgument(msg).handle_error();
            }
            _ => {}
        }
    }
}

/// Prints the help information for available commands.
fn print_help() {
    println!("Available commands:");
    for cmd in Command::iter() {
        println!("- {}", cmd.to_string().to_lowercase().trim());
    }
    println!();
}

/// Parses the input string and returns a 'Command' enum value or an error.
fn parse_command(input: &str) -> Result<Command, CommandError> {
    let mut parts = input.trim().split_whitespace();

    let cmd_str = parts.next().ok_or(CommandError::EmptyInput)?;

    match cmd_str {
        "help" => Ok(Command::Help),
        "quit" => Ok(Command::Quit),
        _ => Err(CommandError::UnknownCommand(cmd_str.to_string())),
    }
}
