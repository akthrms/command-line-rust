use clap::Parser;

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust cat"
)]
pub struct Config {
    /// Input file(s)
    #[clap(value_name = "FILE", multiple_occurrences = true, default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[clap(
        short = 'n',
        long = "number",
        takes_value = false,
        conflicts_with = "number-nonblank-lines"
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[clap(short = 'b', long = "number-nonblank", takes_value = false)]
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> AppResult<()> {
    dbg!(config);
    println!("Hello, world!");
    Ok(())
}
