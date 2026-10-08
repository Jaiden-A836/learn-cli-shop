//! The main loop of the program.

use crate::commands::{CommandRegistry, CommandType, HelpCommand, QuitCommand};
use crate::errors::CommandError;
use std::io::{self, Write};

pub fn main_loop() {
    let mut registry = CommandRegistry::new();
    registry.register(CommandType::Help, Box::new(HelpCommand));
    registry.register(CommandType::Help, Box::new(QuitCommand));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        // Check for EOF (Ctrl+D)
        if bytes_read == 0 {
            println!("\nGoodbye!");
            break;
        }

        // Parse input string into a CommandType
        match parse_command(input.trim()) {
            Ok(cmd_type) => {
                // Dispatch execution to command registry
                if let Err(err) = registry.dispatch(&cmd_type) {
                    eprintln!("{}", err);
                }
            }
            Err(CommandError::EmptyInput) => continue,
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn parse_command(input: &str) -> Result<CommandType, CommandError> {
    let mut parts = input.trim().split_whitespace();

    // Returns `CommandError::EmptyInput` if user hits Enter on blank line
    let cmd_str = parts.next().ok_or(CommandError::EmptyInput)?;

    match cmd_str {
        "help" => Ok(CommandType::Help),
        "quit" => Ok(CommandType::Quit),
        _ => Err(CommandError::UnknownCommand(cmd_str.to_string())),
    }
}
