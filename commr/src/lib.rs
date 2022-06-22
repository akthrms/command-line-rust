use clap::Parser;
use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct App {}

#[derive(Debug)]
enum Column<'a> {
    Column1(&'a str),
    Column2(&'a str),
    Column3(&'a str),
}

impl App {
    pub fn run(self) -> AppResult<()> {
        Ok(())
    }
}
