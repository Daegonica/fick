use clap::Parser;

use fick::FickCLI;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    // Arguments. IE, is it CSV file? Are we looking for case sensitive?
    #[arg(short, long)]
    ignore_case: bool,

    #[arg(short, long)]
    csv: bool,

    // Main file path
    file_path: String,

    // Optional
    query: Option<String>,

    // Secondary Options
    options: Option<String>

}

// cargo run -- [settings] cibc.csv [options]

fn main() {
    let mut fick = FickCLI::new();

    let mut args = Args::parse();
    args.file_path = "records\\".to_owned() + &args.file_path;

    if args.csv {
        if let Err(e) = fick.read_csv(&args.file_path, &args.query, &args.options){
            fick.log.error(&format!("Error reading CSV: {}", e));
        };
    } else {
        read_text(&String::from("Hello from Text!"));
    }

    println!("{:#?}", &args);
}

fn read_text(file_path: &String) {
    println!("{}", file_path);
}