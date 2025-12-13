# Daegonica Software CLI Financial Checker (fick)

A financial checker currently in development. The purpose of this CLI program will to to take in various financial records and parse the information into text files based on optional inputs.

## Features
- Total money spent
- Total money earned
- Subscriptions per period
- Categories per period
 - IE, Bills, Groceries, Online, Games, and other customizable filters.

## Tech
- Rust
- Various crates for reading/writing to files of different types.

## Status 
Active Development

## [query]
- "filters needed"
    - Shows a list of possible patterns not within config.toml with digits removed to make for easier copy/paste
- "show filters"
    - Shows all the filter categories in config.toml that can be used as [category] in [options]
- "total earned"
    - Currently the only implementation of money calculating. Soon to be adding "total spent".

## [options]
- "2025-03-01 2025-06-01"
    - Date range formated as such.
- "[category]"
    - The [category] is manually placed into the config.toml file.
- The placement of the date range and category is interchangable. You can do the date range first then category, or vise-versa.

## How to Run.
```bash
cargo run -- [optional flags: -c(csv file)] file.type [query] [options]
