use csv::{self, StringRecord};
use std::{fs::File, io::Result};

const FILE_PATH: &str = "tasks.txt";

pub fn fetch_tasks() -> Result<Option<Vec<StringRecord>>> {
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
