type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run() -> AppResult<()> {
    println!("Hello, world!");
    Ok(())
}
