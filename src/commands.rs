//! Manages and holds user commands.

use crate::errors::CommandError;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter, Eq, PartialEq, Hash, Copy, Clone)]
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
