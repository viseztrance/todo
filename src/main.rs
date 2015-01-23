// Read argvs

// Read config as todo
// Or create a blank todo

// Save a todo

// Edit a todo

use std::os;
mod cli;
mod todo {
    pub mod list;
    pub mod entry;
}

fn main() {
    let mut path = os::getcwd().unwrap();
    let mut args = os::args();
    cli::delegate(path, args);
}
