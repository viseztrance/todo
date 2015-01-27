use std::io;
use todo::list::List;
use query::Query;

pub fn delegate(mut path: Path, mut args: Vec<String>) {
    // First argument contains the program / file name,
    // so let's discard it to make postprocessing easier
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
                    let parsed_input: Option<usize> = input.parse::<usize>();
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
                    let input = read_input("What's the id of the task that you want to edit?");
                    let parsed_input: Option<usize> = input.parse::<usize>();
                    vec![parsed_input.unwrap()]
                }
            };
            list.remove(ids);
            list.save();
        },
        "finish" => {
            let ids = match query.scope.to_vec_int() {
                Some(val) => val,
                None => {
                    let input = read_input("What's the id of the task that you want to edit?");
                    let parsed_input: Option<usize> = input.parse::<usize>();
                    vec![parsed_input.unwrap()]
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
    io::stdin().read_line().unwrap().trim().to_string()
}

fn render_help() {
    println!("Print help");
}