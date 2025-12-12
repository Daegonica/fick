use crate::prelude::*;

pub fn calc_amount_earned(log: &mut Logger, records: &[Record], options: &Vec<String>) {
    log.info("Calculating totals!");

    let date_range_re = Regex::new(r"^(\d{4}-\d{2}-\d{2}) (\d{4}-\d{2}-\d{2})$").unwrap();
    let mut start_date: Option<NaiveDate> = None;
    let mut end_date: Option<NaiveDate> = None;

    if let Some(first) = options.get(0) {
        if first != "complete" {
            match first {
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
        }
    }

    let mut total_earned: f64 = 0.0;
    let ignore_info = Regex::new(r"Internet Banking INTERNET TRANSFER \d+").unwrap();

    for record in records {

        if !ignore_info.is_match(&record.info) {
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