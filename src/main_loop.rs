//! The main loop of the program.

use crate::commands::{CommandRegistry, CommandType, HelpCommand, QuitCommand};
use crate::errors::CommandError;
use std::io::{self, Write};

pub fn main_loop() {
    let mut registry = CommandRegistry::new();
    registry.register(CommandType::Help, Box::new(HelpCommand));
    registry.register(CommandType::Quit, Box::new(QuitCommand));

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
        match CommandType::try_from(input.trim()) {
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
