use todo::list::List;

pub fn delegate(mut path: Path, mut args: Vec<String>) {
    // First argument contains the program / file name,
    // so let's discard it to make postprocessing easier
    if args.len() > 0 {
        args.remove(0);
    }

    path.push("todo.txt");

    match args.len() {
        0 => render_help(),
        1 => dispatch(path, args[0].clone(), None),
        _ => {
            let (action, context) = process_arguments(args);
            dispatch(path, action, context);
        }
    }
}

fn dispatch(path: Path, action: String, context: Option<String>) {
    let list = List::new(path);

    match action.as_slice() {
        "list"   => list.index(context),
        "add"    => list.add(context),
        "edit"   => list.edit(context),
        "remove" => list.remove(context),
        "finish" => list.finish(context),
        "count"  => list.count(context),
        _        => println!("There is no action named `{}`.", action)
    }
}

fn process_arguments(mut args: Vec<String>) -> (String, Option<String>) {
    let action = args.remove(0);
    let mut context = "".to_string();
    for arg in args.iter() {
        context.push_str(arg.as_slice());
    }
    (action, Some(context))
}

fn render_help() {
    println!("Print help");
}