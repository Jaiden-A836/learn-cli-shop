use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter)]
pub enum Command {
    Help,
    Quit,
    Unknown,
}

impl Command {
    fn parse(input: &str) -> Command {
        match input.trim().to_lowercase().as_str() {
            "help" => Command::Help,
            "quit" => Command::Quit,
            _ => Command::Unknown,
        }
    }
}
