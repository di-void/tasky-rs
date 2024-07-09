use csv::{self, Result as CsvResult, StringRecord};
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

pub fn add_tasks() -> Result<()> {
    let task_id = Uuid::new_v4().to_string();

    let mut record = vec![task_id];

    let prompts = text_prompts(vec![
        "What's the name of the task?",
        "How about a description?",
    ]);

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

pub fn edit_task() -> Result<()> {
    // prompt and get task id
    // get the records from the file
    // check if record with id exists
    // if it doesn't exist, return and print error message
    // if it does, prompt with name change,
    // desc change and status change(options select)
    // write changes to record in csv file
    println!("Edit task :)");
    Ok(())
}

fn text_prompts(txt_prompts: Vec<&str>) -> Vec<std::result::Result<String, InquireError>> {
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
