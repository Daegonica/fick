use std::{fs::File, io::Write, collections::HashMap};
use csv::Reader;
use colored::*;
use regex::Regex;

// Toolbox
use file_reader::*;
use dlog::{*, enums::OutputTarget};

// Fick files
mod filter_parser;

// Constants
const FILTER_PATH: &str = "config\\config.toml";

pub struct FickCLI {
    pub log: Logger,
    file_reader: FileReader,
    records: HashMap<String, Vec<Record>>, // Each Vec<Record> is one CSV file
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

        FickCLI { log, file_reader, records: HashMap::new() }
    }

    pub fn read_csv(&mut self, file_path: &String, query: &Option<String>, options: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.log.info(&format!("Reading from: {}", file_path));
        let mut contents = self.log.log_result(
            Reader::from_path(file_path),
            file_path,
            |log| log.info("Reader successful!")
        )?;

        let mut all_info: Vec<Record> = Vec::new();
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
            all_info.push(current_line);
        }
        self.log.info("All info recorded!");

        // TODO: Add in functions of what we want to do. By date, by transaction, by amount

        Ok(())
    }

    fn calc_amount_earned(&mut self, label: &str) {
        let contents = match self.records.get(label) {
            Some(c) => c,
            None => {
                self.log.error("Missing Records!");
                return;
            }
        };
        let mut total_earned: f64 = 0.0;
        let re_transfer = Regex::new(r"Internet Banking INTERNET TRANSFER (\d+)").unwrap();

        for transaction in contents.iter() {
            let money_in: f64 = transaction.money_in.parse().unwrap_or(0.0);
            if money_in > 0.0 && !re_transfer.is_match(&transaction.info) {
                let info_str = transaction.info.clone();
                let date_str = transaction.date.as_str();

                total_earned += money_in;
                let total_rounded = (total_earned * 100.0).round() / 100.0;
                
                println!("[{}] Earned {} on {}.\n",
                        date_str,
                        money_in.to_string().red(), 
                        info_str.green(), 
                );
                total_earned = total_rounded;
            }
        }

        println!("Total earned: {}", total_earned);
    }

    fn calc_amount_spent(&mut self, label: &str, filter: &Option<String>) {

        // Start total_spent with 0.0 for tracking
        let mut total_spent: f64 = 0.0;
        let contents = match self.records.get(label) {
            Some(c) => c,
            None => {
                self.log.error("Missing Records!");
                return;
            }
        };

        // Set variables for logging info to designated file.
        let f: String = filter.clone().unwrap_or("None".to_string());
        let file_path = format!("C:\\Users\\memph\\Documents\\{}_info.txt", f);
        let mut file = File::create(file_path).unwrap();

        // If using a filter, what will it show?
        let filter_contents = self.file_reader.file_type(FILTER_PATH);

        let toml_str = match filter_contents {
            Ok(FileContent::Text(s)) => s,
            Ok(FileContent::Lines(_)) => {
                eprintln!("Expected FileContent::Text for TOML config, using empty config.");
                String::new()
            }
            Err(e) => {
                eprintln!("Error reading config: {e}, using empty config.");
                String::new()
            }
        };

        let filters = filter_parser::load_filters(&toml_str, &mut self.log).unwrap_or_default();

        // If you don't choose a filter it defaults to this
        let default_regex: Vec<Regex> = filters.iter()
            .flat_map(|(_, regexes, _)| regexes.iter().cloned())
            .collect();
        self.log.info("Defaults loaded!");

        // Removes this text from strings to clean things up a bit
        let remove = [
            Regex::new(r"Point of Sale - Interac RETAIL PURCHASE (\d+) ").unwrap()
        ];
        self.log.info("Set regex to remove!");

        // Iterate over the filters list. If you find one that matches f then map the logic to regex and filter_on_match if not use the default filters
        let (regex, filter_on_match) = filters
            .iter()
            .find(|(name, _, _)| *name== f)
            .map(|(_, regex, logic)| (regex, *logic))
            .unwrap_or_else(|| (&default_regex, false)
        );
        self.log.info("Full Regex patterns loaded!");

        // Iterate over the file given
        for transactions in contents.iter(){
            let money_out_str: f64 = transactions.money_out.parse().unwrap_or(0.0);

            // Use the regex grabbed before from default or the filter and match any string with any regex listed.
            let is_match = regex.iter().any(|re| re.is_match(&transactions.info));

            // If you found the filter category, and the bool is true or its set to false. And this is a money_out string.
            if ((is_match && filter_on_match) || (!is_match && !filter_on_match)) && money_out_str > 0.0 {

                let mut info_str = transactions.info.clone();
                let date_str = transactions.date.as_str();

                total_spent += money_out_str;
                let total_rounded = (total_spent * 100.0).round() / 100.0;

                for rm in remove.iter() {
                    info_str = rm.replace_all(&info_str, "").to_string();
                }

                let _ = writeln!(file, "[{}] Spent {} on {}. Adding it to the total: {}\n",
                        date_str,
                        money_out_str, 
                        info_str, 
                        total_rounded
                );

                total_spent = total_rounded;
            }
        }

        // If successful write collected total to end of file.
        let _ = writeln!(file, "Total spent: {}", total_spent);
    }

    fn sort_csv_by_date(&mut self, label: &str) {
        let contents = match self.records.get(label) {
            Some(c) => c,
            None => {
                self.log.error("Missing Records!");
                return;
            }
        };

        let mut dates: HashMap<String, Vec<(&str, &str, &str)>> = HashMap::new();
        for transactions in contents.iter() {

            let info_str = transactions.info.as_str();
            let money_out_str = transactions.money_out.as_str();
            let money_in_str = transactions.money_in.as_str();

            dates.entry(transactions.date.clone())
                .or_insert(Vec::new())
                .push((info_str, money_out_str, money_in_str));
        }

        //println!("{:#?}", dates);
    }

}