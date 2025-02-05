use clap::command;
use tasky::*;

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(command!("list").about("See your list of tasks"))
        .subcommand(command!("add").about("Add to your tasks list"))
        .subcommand(command!("edit").about("Edit a task in your list"))
        .subcommand(command!("delete").about("Delete a task from your list"))
        .get_matches();

    // sub-commands
    match matches.subcommand() {
        Some(("list", _)) => {
            // fetch tasks
            let tasks = fetch_tasks()
                .expect("An IO error occured!")
                .unwrap_or_else(|| {
                    println!("You do not have any tasks yet!");
                    vec![]
                });

            println!();

            for task in tasks {
                utils::print_tasks(&task, task.len());
            }
        }

        // add a task
        Some(("add", _)) => match add_tasks() {
            Ok(_) => println!("Task written successfully!"),
            Err(err) => println!("An IO error occurred! {:?}", err),
        },

        // edit a task
        Some(("edit", _)) => match edit_task() {
            Ok(_) => println!("Task updated successfully!"),
            Err(err) => println!("An error occurred! {:?}", err),
        },

        // delete a task
        Some(("delete", _)) => delete_task(),

        _ => unreachable!("Exhausted list of subcommands"),
    }
}
