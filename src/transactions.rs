use crate::prelude::*;

pub fn calc_amount(log: &mut Logger, records: &[Record], options: &Vec<String>, filters: &Config, calc_type: String) {
    log.info("Calculating totals!");

    let mut default_search: bool = true;
    let mut start_date: Option<NaiveDate> = None;
    let mut end_date: Option<NaiveDate> = None;
    let mut record_date: Option<NaiveDate> = None;
    let mut regex_filters: Vec<Regex> = Vec::new();

    for opt in options {
        if DATE_FORMAT.is_match(&opt) {
            (start_date, end_date) = set_date_range(opt, log);
            default_search = false;
        }

        for filter in &filters.filters {
            if filter.category == *opt {
                default_search = false;
                for pattern in &filter.patterns {
                    let lower_pattern = pattern.to_lowercase();
                    if let Ok(regex) = Regex::new(&lower_pattern) {
                        regex_filters.push(regex);
                    }
                }
            }
        }
    }

    let mut total: f64 = 0.0;
    let ignore_info = Regex::new(r"Internet Banking INTERNET TRANSFER \d+").unwrap();
    let mut store_info: Vec<(String, String)> = Vec::new();

    for record in records {
        let mut record_info: bool = true;
        let calc_value = match calc_type.as_str() {
            "money_in" => &record.money_in,
            "money_out" => &record.money_out,
            _ => ""
        };
        let lower_info = record.info.to_lowercase();
        
        if !default_search {
            // Need to add logic to handle date range, and optional categories.
            if start_date != None {
                record_date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d").ok();
                if !(record_date >= start_date && record_date <= end_date) {
                    record_info = false;
                }
            }

            // Need to add in category logic
            let matched = regex_filters.iter().any(|re| re.is_match(&lower_info));
            if !matched {
                record_info = false;
            }
        }
        // Isn't doing the totals for some categories. Need to find out why.

        if !ignore_info.is_match(&record.info) && record_info && !calc_value.is_empty() {
            store_info.push((record.info.clone(), record.date.clone()));
            total += match calc_value.parse::<f64>() {
                Ok(value) => {
                    (value * 100.00).round() / 100.0
                },
                Err(_) => 0.0,
            };
        
        }
    }

    log.info(format!("Total: {}", (total *  100.0).round() / 100.0));
    log.info("Sources: ");
    for (info, date) in &store_info {
        log.info(format!("{} - {}", date, info));
    }
}

fn set_date_range(range: &String, log: &mut Logger) -> (Option<NaiveDate>, Option<NaiveDate>) {

    let mut start_date: Option<NaiveDate> = None;
    let mut end_date: Option<NaiveDate> = None;

    match range {
        opt if DATE_FORMAT.is_match(opt) => {
            let caps = DATE_FORMAT.captures(opt).unwrap();
            start_date = NaiveDate::parse_from_str(&caps[1], "%Y-%m-%d").ok();
            end_date = NaiveDate::parse_from_str(&caps[2], "%Y-%m-%d").ok();
            log.info("Setting date range.");
        }
         _ => {
            log.info("Invalid date range format. Ignoring filters. Going for complete data.");
        }
    };

    (start_date, end_date)
}
    