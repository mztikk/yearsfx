use atty::Stream;
use chrono::Datelike;
use std::io::{self, BufRead};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    start_year: i32,
    end_year: Option<i32>,
}

fn main() {
    if atty::is(Stream::Stdin) {
        return;
    }

    let args = Cli::from_args();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for i in args.start_year..=args.end_year.unwrap_or_else(|| chrono::Utc::now().year()) {
            println!("{}{}", line, i);
        }
    }
}
