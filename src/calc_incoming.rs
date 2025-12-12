use crate::prelude::*;

pub fn calc_amount_earned(log: &mut Logger, records: &[Record], options: &Vec<String>) {
    log.info("Calculating totals!");

    let mut default_search: bool = true;
    let mut start_date: Option<NaiveDate> = None;
    let mut end_date: Option<NaiveDate> = None;
    let mut record_date: Option<NaiveDate> = None;

    if let Some(first) = options.get(0) {
        if first != "complete" {
            (start_date, end_date) = set_date_range(first, log);
            default_search = false;
        }
    }

    let mut total_earned: f64 = 0.0;
    let ignore_info = Regex::new(r"Internet Banking INTERNET TRANSFER \d+").unwrap();

    for record in records {
        let mut record_info: bool = true;
        
        if !default_search {
            // Need to add logic to handle date range, and optional categories.
            record_date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d").ok();
            if !(record_date >= start_date && record_date <= end_date) {
                record_info = false;
            }
        }
                
        if !ignore_info.is_match(&record.info) && record_info {
            total_earned += match record.money_in.parse::<f64>() {
                Ok(value) => {
                    log.info(format!("{}", record.date));
                    (value * 100.00).round() / 100.0
                },
                Err(_) => 0.0,
            };
        
        }

    }
    log.info(format!("Total Earned: {}", (total_earned *  100.0).round() / 100.0));
}

fn set_date_range(range: &String, log: &mut Logger) -> (Option<NaiveDate>, Option<NaiveDate>) {

    let date_range_re = Regex::new(r"^(\d{4}-\d{2}-\d{2}) (\d{4}-\d{2}-\d{2})$").unwrap();
    let mut start_date: Option<NaiveDate> = None;
    let mut end_date: Option<NaiveDate> = None;

    match range {
        opt if date_range_re.is_match(opt) => {
            let caps = date_range_re.captures(opt).unwrap();
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
    