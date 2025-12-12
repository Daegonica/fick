mod calc_incoming;
mod prelude;

use std::{fs::File, io::Write, collections::HashMap};
use csv::Reader;

// Toolbox
use file_reader::*;
use crate::prelude::*;

// Constants
const FILTER_PATH: &str = "config\\config.toml";

pub struct FickCLI {
    pub log: Logger,
    file_reader: FileReader,
    records: Vec<Record>, // Each Vec<Record> is one CSV file
}

#[derive(Debug)]
pub struct Record {
    date: String,
    info: String,
    money_out: String,
    money_in: String
}

impl FickCLI {
    pub fn new() -> Self {
        let log = Logger::init("fick", None, OutputTarget::Terminal).unwrap();
        let file_reader = FileReader::new("Terminal");

        FickCLI { log, file_reader, records: Vec::new() }
    }

    pub fn read_csv(&mut self, file_path: &String, query: &Option<String>, options: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        
        self.log.info(&format!("Reading from: {}", file_path));
        let v_contents = self.file_reader.file_type(file_path);
        let binding = v_contents?;
        let table = binding.as_table().unwrap();

        self.log.info("Placing info into Vector");
        for r in table {
            let c_l = Record {
                date: r[0].to_string(),
                info: r[1].to_string(),
                money_out: r[2].to_string(),
                money_in: r[3].to_string(),
            };
            self.records.push(c_l);
        }
        self.log.info("All info recorded!");

        let mut default_option = "complete";

        match options.as_deref() {
            Some(option) => {
                default_option = option;
            },
            _ => self.log.info("Didn't pick an option! Defaulting to complete!")
        }

        // TODO: Add in functions of what we want to do. By date, by transaction, by amount
        match query.as_deref() {
            Some("total earned") => calc_incoming::calc_amount_earned(&mut self.log, &self.records, default_option),
            _ => self.log.info("Didn't pick a query!"),
        }

        Ok(())
    }
 
}