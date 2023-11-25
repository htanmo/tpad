use std::env;
use tpad::{Task, HELP};

fn main() {
    let mut task = Task::new();
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    match args.len() {
        1 => task.list(),
        2 => match command {
            "init" => task.init(),
            "list" => task.list(),
            "reset" => task.reset(),
            "restore" => task.restore(),
            "help" | "--help" | "-h" | _ => println!("{}", HELP),
        },
        _ => match command {
            "add" => task.add(&args[2..]),
            "done" => task.done(&args[2..]),
            "undo" => task.undo(&args[2..]),
            "rm" => task.remove(&args[2..]),
            _ => println!("{}", HELP),
        },
    }
}
