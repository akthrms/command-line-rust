use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;
use walkdir::WalkDir;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq)]
enum EntryType {
    File,
    Dir,
    Link,
}

impl FromStr for EntryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(Self::Dir),
            "f" => Ok(Self::File),
            "l" => Ok(Self::Link),
            _ => Err(format!("invalid type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust find"
)]
pub struct App {
    /// Search paths
    #[clap(value_name = "PATHS", default_value = ".", multiple = true)]
    paths: Vec<String>,

    /// Name
    #[clap(
        short = 'n',
        long = "name",
        value_name = "NAME",
        takes_value = true,
        multiple = true
    )]
    names: Vec<Regex>,

    /// Entry type
    #[clap(
        short = 't',
        long = "type",
        value_name = "TYPE",
        takes_value = true,
        multiple = true,
        possible_values = &["f", "d", "l"]
    )]
    entry_types: Vec<EntryType>,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        for path in &self.paths {
            let entries = WalkDir::new(path)
                .into_iter()
                .filter_map(|entry| match entry {
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                    Ok(entry) => Some(entry),
                })
                .filter(|entry| {
                    self.entry_types.is_empty()
                        || self.entry_types.iter().any(|entry_type| match entry_type {
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::Link => entry.file_type().is_symlink(),
                        })
                })
                .filter(|entry| {
                    self.names.is_empty()
                        || self
                            .names
                            .iter()
                            .any(|name| name.is_match(&entry.file_name().to_string_lossy()))
                })
                .map(|entry| entry.path().display().to_string())
                .collect::<Vec<_>>();

            println!("{}", entries.join("\n"));
        }

        Ok(())
    }
}
