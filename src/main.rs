#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

use std::os;
mod cli;
mod todo {
    pub mod list;
    pub mod entry;
}

fn main() {
    let path = os::getcwd().unwrap();
    let args = os::args();
    cli::delegate(path, args);
}
