use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter)]
pub enum Command {
    Help,
    Quit,
}
