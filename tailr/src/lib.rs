use std::error::Error;

type AppResult<T> = Result<T, Box<dyn Error>>;

pub struct App {}

impl App {
    pub fn run(self) -> AppResult<()> {
        Ok(())
    }
}
