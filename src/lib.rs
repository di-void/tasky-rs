use csv::{self, Result as CsvResult, StringRecord};
use inquire::{validator::Validation, InquireError, Select, Text};
use std::{
    fs::{File, OpenOptions},
    io::{self, Error as IOError, ErrorKind, Result as ioResult},
};
use uuid::Uuid;

const FILE_PATH: &str = "tasks.txt";
const HEADERS: [&str; 4] = ["Id", "Name", "Description", "Status"];

fn get_file(append: bool, truncate: bool) -> ioResult<File> {
    OpenOptions::new()
        .append(append)
        .truncate(truncate)
        .write(true)
        .create(true)
        .open(FILE_PATH)
}

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
                if let Some((idx, record)) = records
                    .iter()
                    .enumerate()
                    .find(|(_, r)| r.iter().collect::<Vec<_>>()[0] == id)
                {
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
                        Err(err) => println!("An error occured!: {:?}", err),
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

// Vec<(prompt, default)>
fn text_prompts(txt_prompts: Vec<(&str, &str)>) -> Vec<Result<String, InquireError>> {
    let validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid("Too much text boss.".into()))
        } else if input.trim() == "" {
            Ok(Validation::Invalid("Stop playing boss.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let mut prompts = vec![];

    for msg in txt_prompts {
        let txt_prmpt = Text::new(msg.0)
            .with_default(msg.1)
            .with_validator(validator)
            .prompt();

        prompts.push(txt_prmpt)
    }

    prompts
}

pub fn print_tasks(record: &StringRecord, fields_len: usize) {
    for i in 0..fields_len {
        let field_record = record.get(i).unwrap();

        println!("{:?}: {:?}", HEADERS[i], field_record);
    }

    println!("\n");
}
