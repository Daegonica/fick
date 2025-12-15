use clap::Parser;

use fick::FickCLI;

mod tui;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    // Main file path
    file_path: String,

    // Optional
    #[clap()]
    query: String,

    // Secondary Options
    #[clap()]
    options: Vec<String>

}

// cargo run -- [settings] cibc.csv [options]

fn main() {
    let mut fick = FickCLI::new();

    let mut args = Args::parse();
    args.file_path = "records\\".to_owned() + &args.file_path;

    if args.query != "tui" {
        if let Err(e) = fick.read_csv(&args.file_path, &args.query, &args.options){
            fick.log.error(&format!("Error reading CSV: {}", e));
        };
    } else {
        let _ = tui::draw();
    }

}
