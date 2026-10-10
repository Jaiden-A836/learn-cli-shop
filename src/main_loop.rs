//! The main loop of the program.

use crate::commands::{CommandRegistry, CommandType, HelpCommand, QuitCommand};
use crate::errors::CommandError;
use anyhow::{Context, Result};
use std::io::{self, BufRead, Write};

pub fn main_loop() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_loop(stdin.lock(), stdout).context("Fatal error in application main loop.")
}

/// Core loop that works with any input and output stream.
pub fn run_loop<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> Result<()> {
    let mut registry = CommandRegistry::new();
    registry.register(CommandType::Help, Box::new(HelpCommand));
    registry.register(CommandType::Quit, Box::new(QuitCommand));

    loop {
        write!(writer, "> ").context("Failed to write prompt symbol.")?;
        writer.flush().context("Failed to flush writer buffer.")?;

        let mut input = String::new();

        // Read input
        let bytes_read = reader
            .read_line(&mut input)
            .context("Failed to read line from input stream.")?;

        // Check for EOF (Ctrl+D)
        if bytes_read == 0 {
            writeln!(writer, "\nGoodbye!")?;
            break Ok(());
        }

        // Parse input string into a CommandType
        match CommandType::try_from(input.trim()) {
            Ok(cmd_type) => {
                // Dispatch execution to command registry
                if let Err(err) = registry.dispatch(&mut writer, &cmd_type) {
                    writeln!(writer, "{:?}", err)?;
                }
            }
            Err(CommandError::EmptyInput) => continue,
            Err(err) => writeln!(writer, "{}", err)?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;
    use rstest::rstest;

    #[rstest]
    #[case("help\n", "Available commands:")]
    #[case("bad_cmd\n", "Unknown command:")]
    #[case("\n", "> ")]
    #[case("", "Goodbye!")]
    fn run_loop_behavior(#[case] input: &str, #[case] expected_output: &str) {
        let mut output_buffer = Vec::new();
        run_loop(input.as_bytes(), &mut output_buffer).unwrap();

        let output_str = String::from_utf8(output_buffer).unwrap();

        // assert_eq!(output_str, expected_output);
        assert!(
            output_str.contains(expected_output),
            "Expected:\n{:?}\nGot:\n{:?}",
            expected_output,
            output_str,
        )
    }

    proptest! {
        #[test]
        fn run_loop_never_panics_on_arbitrary_input(random_input in "\\PC*") {
            // Guard: don't let proptest trigger std::process::exit(0)
            if random_input.trim().eq_ignore_ascii_case("quit") {
                return Ok(());
            }

            let mut output_buffer = Vec::new();

            run_loop(random_input.as_bytes(), &mut output_buffer).unwrap();
        }
    }
}
