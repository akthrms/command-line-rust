use clap::Parser;
use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust head"
)]
pub struct App {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        println!("{:#?}", self);
        Ok(())
    }
}
