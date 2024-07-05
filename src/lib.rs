use csv::{self, StringRecord};
use inquire::{validator::Validation, InquireError, Text};
use std::{fs::File, io::Result};
use uuid::Uuid;

const FILE_PATH: &str = "tasks.txt";
const HEADERS: [&str; 4] = ["Id", "Name", "Description", "Status"];

pub fn fetch_tasks() -> Result<Option<Vec<StringRecord>>> {
    println!("Getting tasks...");

    match File::create_new(FILE_PATH) {
        Ok(new_file) => {
            // write csv headers to tasks file
            let mut wtr = csv::Writer::from_writer(new_file);
            wtr.write_record(&HEADERS)?;
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

// prompt user twice for name and desc
// generate uuid for task
// build the writer
// write to csv data store
pub fn add_tasks() -> Result<()> {
    let task_id = Uuid::new_v4().to_string();

    let mut record = vec![task_id];

    let prompts = text_prompts(["What's the task?", "How about a task description:"]);

    for prompt in prompts {
        match prompt {
            Ok(res) => {
                record.push(res);
            }
            Err(err) => println!("Error during inquiry: {:?}", err),
        }
    }

    record.push("Pending".to_string());

    // write to csv

    let mut wtr = csv::Writer::from_path(FILE_PATH)?;

    wtr.write_record(&HEADERS)?;
    wtr.write_record(&record)?;
    wtr.flush()?;

    Ok(())
}

fn text_prompts(msgs: [&str; 2]) -> Vec<std::result::Result<String, InquireError>> {
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

    for msg in msgs {
        let txt_prmpt = Text::new(msg)
            .with_default("")
            .with_validator(validator)
            .prompt();

        prompts.push(txt_prmpt)
    }

    prompts
}

pub fn tasks_printer(record: &StringRecord, fields_len: usize) {
    println!("\n");

    for i in 0..fields_len {
        let field_record = record.get(i).unwrap();

        println!("{:?}: {:?}", HEADERS[i], field_record);
    }
}
