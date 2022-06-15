use clap::Parser;
use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
pub struct App {}

impl App {
    pub fn run(self) -> AppResult<()> {
        Ok(())
    }
}
