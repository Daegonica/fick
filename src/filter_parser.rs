use serde::Deserialize;
use toml;
use regex::Regex;

use dlog::*;

#[derive(Debug, Deserialize)]
pub struct FilterConfig {
    pub category: String,
    pub patterns: Vec<String>,
    pub filter_on_match: bool,
}

#[derive(Debug, Deserialize)]
pub struct FiltersFile {
    pub filters: Vec<FilterConfig>,
}


pub fn load_filters(toml_str: &str, log: &mut Logger) -> Result<Vec<(String, Vec<Regex>, bool)>, Box<dyn std::error::Error>> {

    log.info("Parsing TOML!");
    let filters_file: FiltersFile = toml::from_str(toml_str)?;
    //log.info(&format!("Parsed filters: {:?}", filters_file));

    let filters = filters_file.filters.into_iter().map(|f| {
        let regexes = f.patterns.iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect::<Vec<_>>();
        //log.info(&format!("Cat: {}, Reg: {:?}", f.category, regexes));
        (f.category, regexes, f.filter_on_match)
    }).collect();

    log.info("TOML Filters Parsed!");
    Ok(filters)
}