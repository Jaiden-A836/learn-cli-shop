//! Manages and holds user commands.

use crate::errors::CommandError;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter, Eq, PartialEq, Hash)]
pub enum CommandType {
    Help,
    Quit,
}

/// Shared data or metadata passed to commands during execution.
pub struct CommandContext {
    /// List of available command names and their descriptions for help menus
    pub registered_info: Vec<(&'static str, &'static str)>,
}

pub struct HelpCommand;
pub struct QuitCommand;

/// Manages commands
pub struct CommandRegistry {
    commands: HashMap<CommandType, Box<dyn Command>>,
}

pub trait Command {
    fn execute(&mut self, ctx: &CommandContext) -> Result<(), CommandError>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

impl TryFrom<&str> for CommandType {
    type Error = CommandError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut parts = input.trim().split_whitespace();

        // Returns `CommandError::EmptyInput` if user hits Enter on blank line
        let cmd_str = parts.next().ok_or(CommandError::EmptyInput)?.to_lowercase();

        match cmd_str.as_str() {
            "help" => Ok(CommandType::Help),
            "quit" => Ok(CommandType::Quit),
            _ => Err(CommandError::UnknownCommand(cmd_str.to_string())),
        }
    }
}

impl Command for HelpCommand {
    fn execute(&mut self, ctx: &CommandContext) -> Result<(), CommandError> {
        if CommandType::iter().count() == 0 {
            return Err(CommandError::NoCommandsAvailable);
        }

        println!("Available commands:");
        for (name, desc) in &ctx.registered_info {
            println!("- {:<8} : {}", name, desc);
        }
        println!();

        Ok(())
    }

    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Prints all available commands."
    }
}

impl Command for QuitCommand {
    fn execute(&mut self, _: &CommandContext) -> Result<(), CommandError> {
        println!("Quitting...");
        std::process::exit(0);
    }

    fn name(&self) -> &'static str {
        "quit"
    }

    fn description(&self) -> &'static str {
        "Quit the application."
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd_type: CommandType, command: Box<dyn Command>) {
        self.commands.insert(cmd_type, command);
    }

    /// Executes a command by looking up its CommandType key
    pub fn dispatch(&mut self, cmd_type: &CommandType) -> Result<(), CommandError> {
        // Build the context dynamically from currently registered commands
        let info = self
            .commands
            .values()
            .map(|cmd| (cmd.name(), cmd.description()))
            .collect::<Vec<(&str, &str)>>();

        let ctx = CommandContext {
            registered_info: info,
        };

        if let Some(cmd) = self.commands.get_mut(cmd_type) {
            cmd.execute(&ctx)
        } else {
            Err(CommandError::UnknownCommand(cmd_type.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err_eq, assert_ok};
    use rstest::rstest;

    #[rstest]
    #[case("help", Ok(CommandType::Help))]
    #[case("  QUIT \n", Ok(CommandType::Quit))]
    #[case("   ", Err(CommandError::EmptyInput))]
    #[case("unknown_cmd", Err(CommandError::UnknownCommand("unknown_cmd".to_string())))]
    fn command_try_from(#[case] input: &str, #[case] expected: Result<CommandType, CommandError>) {
        assert_eq!(CommandType::try_from(input), expected);
    }

    #[test]
    fn dispatch_registered_command_succeeds() {
        let mut registry = CommandRegistry::new();
        registry.register(CommandType::Help, Box::new(HelpCommand));

        assert_ok!(registry.dispatch(&CommandType::Help));
    }

    #[test]
    fn dispatch_registered_command_returns_error() {
        let mut registry = CommandRegistry::new();

        assert_err_eq!(
            registry.dispatch(&CommandType::Quit),
            CommandError::UnknownCommand("Quit".to_string())
        );
    }
}
