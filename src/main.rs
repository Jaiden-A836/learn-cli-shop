//! The main starting point of the program.

pub mod commands;
pub mod errors;
pub mod main_loop;
pub mod state;
pub mod tags;

use anyhow::Result;

fn main() -> Result<()> {
    main_loop::main_loop()?;
    Ok(())
}
