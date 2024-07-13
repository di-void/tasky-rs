pub mod utils;
use csv::{self, Result as CsvResult, StringRecord};
use inquire::{InquireError, Select};
use std::io::{self, Error as IOError, ErrorKind, Result as ioResult};
use utils::{find_record, get_file, text_prompts, FILE_PATH, HEADERS};
use uuid::Uuid;

pub fn fetch_tasks() -> ioResult<Option<Vec<StringRecord>>> {
    println!("Getting tasks...");

    match csv::Reader::from_path(FILE_PATH) {
        Ok(mut rdr) => {
            let tasks: Vec<_> = rdr.records().collect::<CsvResult<_>>()?;

            if tasks.len() == 0 {
                return Ok(None);
            }

            return Ok(Some(tasks));
        }
        Err(err) => match err.kind() {
            // check for NotFound IO error
            csv::ErrorKind::Io(err) => {
                if let ErrorKind::NotFound = err.kind() {
                    return Ok(None);
                }
            }
            _ => return Err(io::Error::new(ErrorKind::Other, "Oh no!")),
        },
    };

    // something went wrong somewher
    Err(io::Error::new(ErrorKind::Other, "Something went wrong"))
}

pub fn add_tasks() -> ioResult<()> {
    let task_id = Uuid::new_v4().to_string();

    let mut record = vec![task_id];

    let prompts = text_prompts(vec![
        ("What's the name of the task?", ""),
        ("How about a description?", ""),
    ]);

    for prompt in prompts {
        match prompt {
            Ok(res) => {
                record.push(res);
            }
            Err(err) => {
                println!("Error during inquiry: {:?}", err);
                // need to return an error
                return Err(io::Error::new(ErrorKind::Other, "Error during inquiry"));
            }
        }
    }

    record.push("Pending".to_string());

    match get_file(true, false) {
        Ok(fh) => {
            let meta = fh.metadata()?;
            let mut wtr = csv::Writer::from_writer(fh);

            if meta.len() == 0 {
                wtr.write_record(&HEADERS)?;
            }

            wtr.write_record(&record)?;
            wtr.flush()?;
        }

        // need to return an error
        // but idc rn
        Err(err) => println!("An error occured!: {:?}", err),
    }

    Ok(())
}

pub fn edit_task() -> Result<(), InquireError> {
    let prompts = text_prompts(vec![("What's the task id?", "")]);

    match &prompts[0] {
        Ok(id) => {
            let tasks = fetch_tasks().expect("An IO error occured!");

            if let Some(records) = tasks {
                if let Some((idx, record)) = find_record(&records, id) {
                    let name = record.get(1).unwrap_or("");
                    let desc = record.get(2).unwrap_or("");
                    let status = record.get(3).unwrap_or("Pending");

                    let prompts = text_prompts(vec![
                        ("Update the name?", name),
                        ("update the description?", desc),
                    ]);

                    let mut new_record = vec![id.clone()];

                    for prompt in prompts {
                        let val = prompt?;
                        new_record.push(val);
                    }

                    // "Pending" | "Completed"
                    let start_cursor = if status == "Pending" { 0 } else { 1 };
                    let ans = Select::new("Update the status?", vec!["Pending", "Completed"])
                        .with_starting_cursor(start_cursor)
                        .prompt()?;

                    new_record.push(String::from(ans));

                    let new_records: Vec<_> = records
                        .iter()
                        .enumerate()
                        .map(|(i, rec)| {
                            if i == idx {
                                StringRecord::from(new_record.clone())
                            } else {
                                rec.to_owned()
                            }
                        })
                        .collect();

                    match get_file(false, true) {
                        Ok(fh) => {
                            let mut wtr = csv::Writer::from_writer(fh);

                            wtr.write_record(&HEADERS).expect("Couldn't write reecord!");

                            for record in new_records {
                                wtr.write_record(&record).expect("Couldn't write record");
                            }

                            wtr.flush()?;
                        }
                        Err(err) => panic!("An error occured!: {:?}", err),
                    }
                } else {
                    return Err(InquireError::IO(IOError::new(
                        ErrorKind::Other,
                        "Task not found!",
                    )));
                };
            }
        }
        Err(err) => println!("Error during inquiry: {:?}", err),
    }

    Ok(())
}

pub fn delete_task() {
    let prompts = text_prompts(vec![("What's the task id?", "")]);

    match &prompts[0] {
        Ok(id) => {
            let tasks = fetch_tasks().expect("An IO error occured!");

            if let Some(records) = tasks {
                match find_record(&records, id) {
                    Some(_) => {
                        // filter
                        let new_records = records
                            .iter()
                            .filter(|&rec| rec.get(0).unwrap_or("") != id)
                            .collect::<Vec<_>>();

                        let fh = get_file(false, true).expect("Cound't get file");
                        let mut wtr = csv::Writer::from_writer(fh);

                        wtr.write_record(&HEADERS).expect("Couldn't write reecord!");

                        for record in new_records {
                            wtr.write_record(record).expect("Couldn't write record");
                        }

                        wtr.flush().expect("Cound't flush writer buffer");

                        println!("Task deleted successfully!");
                    }

                    None => println!("Task Not Found!"),
                }
            }
        }

        // i'm too tired
        Err(err) => panic!("Error during inquiry: {:?}", err),
    }
}
