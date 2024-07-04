use clap::command;
use csv::{self, StringRecord};
use std::{fs::File, io::Result};

const FILE_PATH: &str = "tasks.txt";

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

fn fetch_tasks() -> Result<Option<Vec<StringRecord>>> {
    println!("Getting tasks...");

    match File::create_new(FILE_PATH) {
        Ok(new_file) => {
            // write csv headers to tasks file
            let mut wtr = csv::Writer::from_writer(new_file);
            wtr.write_record(&["Id", "Name", "Description", "Status"])?;
            wtr.flush()?;

            return Ok(None);
        }
        Err(_) => {
            let mut rdr = csv::Reader::from_path(FILE_PATH)?;
            let mut tasks = vec![];

            for res in rdr.records() {
                let record = res?;
                tasks.push(record);
            }

            if tasks.len() == 0 {
                return Ok(None);
            }

            return Ok(Some(tasks));
        }
    }
}
