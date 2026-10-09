//! Holds all errors within the program.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Please provide a command.")]
    EmptyInput,

    #[error("No commands available.")]
    NoCommandsAvailable,

    #[error("Unknown command: '{0}'. Type 'help'.")]
    UnknownCommand(String),

    #[error("Missing argument: '{0}'")]
    MissingArgument(String),
}
