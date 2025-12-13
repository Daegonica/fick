pub use regex::Regex;
pub use chrono::NaiveDate;
pub use dlog::{*, enums::OutputTarget};
pub use crate::Record;
pub use crate::Config;
pub use crate::Filter;
use once_cell::sync::Lazy;

pub static DATE_FORMAT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4}-\d{2}-\d{2}) (\d{4}-\d{2}-\d{2})$").unwrap());