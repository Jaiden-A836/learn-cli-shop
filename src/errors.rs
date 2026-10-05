#[derive(Debug)]
pub enum CommandError {
    EmptyInput,
    UnknownCommand(String),
    #[allow(dead_code)]
    MissingArgument(String),
}

impl CommandError {
    pub fn handle_errors(&self) {
        match self {
            CommandError::EmptyInput => eprintln!("Please provide a command."),
            CommandError::UnknownCommand(cmd) => {
                eprintln!("Unknown command: '{}'. Type 'help'.", cmd);
            }
            CommandError::MissingArgument(msg) => {
                eprintln!("Missing argument: '{}'", msg);
            }
        }
    }
}
