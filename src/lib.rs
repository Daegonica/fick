mod calc_incoming;
mod prelude;

use std::{fs::File, io::Write};
use serde::Deserialize;

// Toolbox
use file_reader::*;
use crate::prelude::*;

// Constants
const FILTER_PATH: &str = "config\\config.toml";

pub struct FickCLI {
    pub log: Logger,
    file_reader: FileReader,
    records: Vec<Record>, // Each Vec<Record> is one CSV file
    filters: Config,
}

#[derive(Debug, Clone)]
pub struct Record {
    date: String,
    info: String,
    money_out: String,
    money_in: String
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Filter {
    category: String,
    patterns: Vec<String>,
    filter_on_match: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    filters: Vec<Filter>,
}


impl FickCLI {
    pub fn new() -> Self {
        let log = Logger::init("fick", None, OutputTarget::Terminal).unwrap();
        let file_reader = FileReader::new("Terminal");

        FickCLI { log, file_reader, records: Vec::new(), filters: Config::default() }
    }

    pub fn read_csv(&mut self, file_path: &String, query: &String, options: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        
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

        let _ = self.load_filters();
        let _ = self.compare_filters_to_data();

        let mut default_option: Vec<String> = Vec::new();
        if options.is_empty() {
            default_option.push("complete".to_string());
        } else {
            default_option = options.clone()
        }


        // TODO: Add in functions of what we want to do. By date, by transaction, by amount
        match query.as_str() {
            "total earned" => calc_incoming::calc_amount_earned(&mut self.log, &self.records, &default_option, &self.filters),
            _ => self.log.info("Didn't pick a query!"),
        }

        Ok(())
    }

    fn compare_filters_to_data(&mut self) {

        let mut records: Vec<String> = Vec::new();
        let re = Regex::new(r"\d+").unwrap();

        for record in self.records.clone() {

            // Need to compare the config.toml patterns with each record.info string before normalizing.
            // We only want to normalize and save the info if we don't currently have a pattern for it.
            for filter in &self.filters.filters {
                // Each 'filter' has a set of patterns.
                    // Each set of patterns need to be checked against every 'record'
                        // 'patterns' are in String format.
                        // Need to convert to regex or compare against String if not.
                    // 'record' holds the 'info', 'money_in', 'money_out'
                    // If any of the patterns in the 'filter' matches the 'info' we want to ignore it
                    // If none of the patterns match the 'info' then we want to mark it storage
                let mut string_patterns: Vec<String> = Vec::new();
                let mut regex_patterns: Vec<Regex> = Vec::new();
                
                for pattern in &filter.patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        regex_filters.push(regex);
                    } else {
                        string_filters.push(pattern.clone());
                    }
                }
            }

            // Normalizes all info lines and places into a Vector ensuring there are no doubles.
            let normalized = re.replace_all(&record.info, "").to_string();
            if !records.contains(&normalized) {
                records.push(normalized);
            }

        }

        self.log.info(format!("{:#?}", records));

    }

    fn load_filters(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        self.log.info("Loading filters!");
        let toml_config = self.file_reader.file_type(FILTER_PATH)?;

        let toml_config = match toml_config {
            FileContent::Text(s) => s,
            _ => return Ok(self.log.error(format!("Expected FileContent::Text"))),
        };

        self.filters = self.log.log_result(
            toml::from_str::<Config>(&toml_config),
            FILTER_PATH,
            |log| log.info("Filters loaded!")
        )?;

        Ok(())
    }
}