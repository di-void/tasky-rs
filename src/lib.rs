use csv::{self, StringRecord};
use inquire::{validator::Validation, InquireError, Text};
use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind, Result},
};
use uuid::Uuid;

const FILE_PATH: &str = "tasks.txt";
const HEADERS: [&str; 4] = ["Id", "Name", "Description", "Status"];

fn get_file() -> Result<File> {
    OpenOptions::new().append(true).create(true).open(FILE_PATH)
}

pub fn fetch_tasks() -> Result<Option<Vec<StringRecord>>> {
    println!("Getting tasks...");

    match csv::Reader::from_path(FILE_PATH) {
        Ok(mut rdr) => {
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

pub fn add_tasks() -> Result<()> {
    let task_id = Uuid::new_v4().to_string();

    let mut record = vec![task_id];

    let prompts = text_prompts("Enter the task name:", "How about a description:");

    for prompt in prompts {
        match prompt {
            Ok(res) => {
                record.push(res);
            }
            Err(err) => println!("Error during inquiry: {:?}", err),
        }
    }

    record.push("Pending".to_string());

    match get_file() {
        Ok(fh) => {
            let meta = fh.metadata()?;
            let mut wtr = csv::Writer::from_writer(fh);

            if meta.len() == 0 {
                wtr.write_record(&HEADERS)?;
            }

            wtr.write_record(&record)?;
            wtr.flush()?;
        }
        Err(err) => println!("An error occured!: {:?}", err),
    }

    Ok(())
}

fn text_prompts(
    task_name: &str,
    task_desc: &str,
) -> Vec<std::result::Result<String, InquireError>> {
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
    let msgs = [task_name, task_desc];

    for msg in msgs {
        let txt_prmpt = Text::new(msg)
            .with_default("")
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
