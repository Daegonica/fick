use std::{fs::File, io::Write, collections::HashMap};
use csv::Reader;
use regex::Regex;
use chrono::NaiveDate;

// Toolbox
use file_reader::*;

mod prelude;
use crate::prelude::*;

// Constants
const FILTER_PATH: &str = "config\\config.toml";

pub struct FickCLI {
    pub log: Logger,
    file_reader: FileReader,
    records: Vec<Record>, // Each Vec<Record> is one CSV file
}

#[derive(Debug)]
struct Record {
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
        let mut contents = self.log.log_result(
            Reader::from_path(file_path),
            file_path,
            |log| log.info("Reader successful!")
        )?;

        self.log.info("Placing info into Vector");

        for result in contents.records() {
            let record = result?;
            let fields: Vec<&str> = record.iter().collect();

            let current_line = Record {
                date: fields[0].to_string(),
                info: fields[1].to_string(),
                money_out: fields[2].to_string(),
                money_in: fields[3].to_string(),
            };
            self.records.push(current_line);
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
            Some("total earned") => self.calc_amount_earned(default_option),
            _ => self.log.info("Didn't pick a query!"),
        }

        Ok(())
    }

    fn calc_amount_earned(&mut self, options: &str) {
        self.log.info("Calculating totals!");

        let date_range_re = Regex::new(r"^(\d{4}-\d{2}-\d{2}) (\d{4}-\d{2}-\d{2})$").unwrap();

        let (start_date, end_date) = match options {
            "complete" => {
                self.log.info("Ignoring filters. Going for complete data.");
                (None, None)
            },
            opt if date_range_re.is_match(opt) => {
                let caps = date_range_re.captures(opt).unwrap();
                let start = NaiveDate::parse_from_str(&caps[1], "%Y-%m-%d").ok();
                let end = NaiveDate::parse_from_str(&caps[2], "%Y-%m-%d").ok();
                self.log.info("Setting date range.");
                (start, end)
            }
            _ => {
                self.log.info("Invalid date range formate. Ignoring filters. Going for complete data.");
                (None, None)
            }
        };

        let mut total_earned: f64 = 0.0;
        let ignore_info = Regex::new(r"Internet Banking INTERNET TRANSFER \d+").unwrap();

        for record in &self.records {


            if !ignore_info.is_match(&record.info) {
                total_earned += match record.money_in.parse::<f64>() {
                    Ok(value) => {
                        self.log.info(format!("{}", record.date));
                        (value * 100.00).round() / 100.0
                    },
                    Err(_) => 0.0,
                };
            }
        }
        self.log.info(format!("Total Earned: {}", (total_earned *  100.0).round() / 100.0));
    }
 
}