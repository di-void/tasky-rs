use csv::StringRecord;
use inquire::{validator::Validation, InquireError, Text};
use std::{
    fs::{File, OpenOptions},
    io::Result as ioResult,
};

pub const FILE_PATH: &str = "tasks.txt";
pub const HEADERS: [&str; 4] = ["Id", "Name", "Description", "Status"];

pub fn get_file(append: bool, truncate: bool) -> ioResult<File> {
    OpenOptions::new()
        .append(append)
        .truncate(truncate)
        .write(true)
        .create(true)
        .open(FILE_PATH)
}

// Vec<(prompt, default)>
pub fn text_prompts(txt_prompts: Vec<(&str, &str)>) -> Vec<Result<String, InquireError>> {
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

pub fn find_record<'a>(
    records: &'a Vec<StringRecord>,
    id: &str,
) -> Option<(usize, &'a StringRecord)> {
    records
        .iter()
        .enumerate()
        .find(|(_, r)| r.iter().collect::<Vec<_>>()[0] == id)
}

pub fn print_tasks(record: &StringRecord, fields_len: usize) {
    for i in 0..fields_len {
        let field_record = record.get(i).unwrap();

        println!("{}: {}", HEADERS[i], field_record);
    }

    println!("\n");
}
