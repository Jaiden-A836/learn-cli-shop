//! The main loop of the program.

use crate::commands::{CommandRegistry, CommandType, HelpCommand, QuitCommand};
use crate::errors::CommandError;
use std::io::{self, BufRead, Write};

pub fn main_loop() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_loop(stdin.lock(), stdout)
}

/// Core loop that works with any input and output stream.
pub fn run_loop<R: BufRead, W: Write>(mut reader: R, mut writer: W) {
    let mut registry = CommandRegistry::new();
    registry.register(CommandType::Help, Box::new(HelpCommand));
    registry.register(CommandType::Quit, Box::new(QuitCommand));

    loop {
        write!(writer, "> ").unwrap();
        writer.flush().unwrap();

        let mut input = String::new();

        // Read input
        let bytes_read = reader.read_line(&mut input).unwrap();

        // Check for EOF (Ctrl+D)
        if bytes_read == 0 {
            writeln!(writer, "\nGoodbye!").unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_loop_exits_on_quit_command() {
        let input = "quit\n";
        let mut output = Vec::new();

        run_loop(input.as_bytes(), &mut output);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("> "));
    }

    #[test]
    fn run_loop_prints_error_on_unknown_command() {
        let input = "quit\n";
        let mut output = Vec::new();

        run_loop(input.as_bytes(), &mut output);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("> "));
    }
}
