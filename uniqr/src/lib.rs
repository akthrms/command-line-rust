use clap::Parser;
use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust uniq"
)]
pub struct App {
    /// Input file
    in_file: String,

    /// Output file
    out_file: Option<String>,

    /// Show counts
    count: bool,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        println!("{:#?}", self);
        Ok(())
    }
}
