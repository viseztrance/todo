#![allow(unstable)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex_macros;
extern crate regex;

use std::os;
use std::old_io;
use todo::list::List;
use query::Query;
use query::QueryScope;

mod query;
pub mod todo {
    pub mod list;
    pub mod entry;
}

fn main() {
    let mut path = os::getcwd().unwrap();
    let mut args = os::args();
    args.remove(0);

    path.push("todo.txt");

    match args.len() {
        0 => render_help(),
        _ => dispatch(path, Query::new(args))
    }
}

fn dispatch(path: Path, query: Query) {
    let mut list = List::new(path);
    list.load();

    match query.action.as_slice() {
        "list" => list.index(query.scope.to_string()),
        "add" => {
            let content = match query.scope.to_string() {
                Some(val) => val,
                None => read_input("Write a description for your task:")
            };
            list.add(content);
            list.save();
        },
        "edit" => {
            let id = match query.scope.to_vec_int() {
                Some(val) => val[0],
                None => {
                    let input = read_input("What's the id of the task that you want to edit?");
                    let parsed_input: Result<usize, _> = input.parse::<usize>();
                    parsed_input.unwrap()
                }
            };
            list.edit(id, read_input("Write a description for your task:"));
            list.save();
        },
        "remove" => {
            let ids = match query.scope.to_vec_int() {
                Some(val) => val,
                None => {
                    let input = read_input("Enter the ids of the entries you want to remove:");
                    QueryScope::new(split_string(input)).to_vec_int().unwrap()
                }
            };
            list.remove(ids);
            list.save();
        },
        "finish" => {
            let ids = match query.scope.to_vec_int() {
                Some(val) => val,
                None => {
                    let input = read_input("Enter the ids of the entries you want to remove:");
                    QueryScope::new(split_string(input)).to_vec_int().unwrap()
                }
            };
            list.finish(ids);
            list.save();
        },
        "count" => list.count(query.scope.to_string()),
        _ => println!("There is no action named `{}`.", query.action)
    }
}

fn read_input(greeting: &str) -> String {
    println!("{}", greeting);
    old_io::stdin().read_line().unwrap().trim().to_string()
}

fn split_string(value: String) -> Vec<String> {
    value.split_str(" ")
         .map(|s| s.to_string())
         .collect()
}

fn render_help() {
    println!("Print help");
}
