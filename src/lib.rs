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

        let mut default_option: Vec<String> = Vec::new();
        match options.as_slice() {
            [] => {
                default_option.push("complete".to_string())
            }
            _ => { 
                default_option = options.clone();
            }
        }


        // TODO: Add in functions of what we want to do. By date, by transaction, by amount
        match query.as_str() {
            "total earned" => calc_incoming::calc_amount_earned(&mut self.log, &self.records, &default_option, &self.filters),
            "filters needed" => self.show_filters_to_add(),
            _ => self.log.info("Didn't pick a query!"),
        }

        Ok(())
    }

    pub fn convert_to_regex(&mut self) -> Vec<Regex> {

        let mut regex_patterns: Vec<Regex> = Vec::new();

        for filter in &self.filters.filters {
            for pattern in &filter.patterns {
                let lower_pattern = pattern.to_lowercase();
                if let Ok(regex) = Regex::new(&lower_pattern) {
                    regex_patterns.push(regex);
                    // self.log.info("Found a Regex to match!");
                }
            }
        }

        regex_patterns
    }

    fn show_filters_to_add(&mut self) {

        let mut records: Vec<String> = Vec::new();
        let re = Regex::new(r"\d+").unwrap();

        let regex_patterns = self.convert_to_regex();

        for record in self.records.clone() {
            let info_lower = record.info.to_lowercase();
            if !regex_patterns.iter().any(|re| re.is_match(&info_lower)) {
                
                let normalized = re.replace_all(&record.info, "").to_string();
                if !records.contains(&normalized) {
                    records.push(normalized);
                }
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