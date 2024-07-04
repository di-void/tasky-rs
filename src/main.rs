use clap::command;
use tasky::*;

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(command!("list").about("Lists all your tasks"))
        .get_matches();

    // sub-commands
    match matches.subcommand() {
        Some(("list", _)) => {
            // fetch tasks
            let tasks = fetch_tasks()
                .expect("An IO error occured!")
                .unwrap_or_else(|| {
                    println!("You do not have any tasks yet!");
                    return vec![];
                });

            for task in tasks {
                println!("Task: {:?}", task);
            }
        }
        _ => unreachable!("Exhausted list of subcommands"),
    }
}
