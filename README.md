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

## How to Run.
```bash
cargo run -- [optional flags: -c(csv file) -i(ignore case sensitive)] file.type [filters] [optional: (features to be added)]
