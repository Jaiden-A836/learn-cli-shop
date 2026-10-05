mod commands;
mod main_loop;
mod errors;

use std::io;

fn main() {
    main_loop::main_loop();
}
