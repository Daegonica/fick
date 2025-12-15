# Daegonica Software CLI Financial Checker (fick)

A financial checker currently in development. The purpose of this CLI program will to to take in various financial records and parse the information into text files based on optional inputs.

## Features
- Automatically detects file types
- Total money sent
- Total money received
- Customizable Categories
- Date range search
- TUI powered by ratatui(Currently being worked on)

## Tech
- Rust
- Various crates for reading/writing to files of different types.

## Status 
Active Development

## [query]
- "tui"
    - For those less command-line savvy there is a TUI option.
- "filters needed"
    - Shows a list of possible patterns not within config.toml with digits removed to make for easier copy/paste
- "show filters"
    - Shows all the filter categories in config.toml that can be used as [category] in [options]
- "total received"
- "total sent"

## [options]
- "2025-03-01 2025-06-01"
    - Date range formated as such.
- "[category]"
    - The [category] is manually placed into the config.toml file.
- The placement of the date range and category is interchangable. You can do the date range first then category, or vise-versa.

## How to Run.
```bash
cargo run -- file.type [query] [options]
