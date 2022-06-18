use clap::{ErrorKind, IntoApp, Parser};
use regex::{Regex, RegexBuilder};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};
use walkdir::WalkDir;

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust grep"
)]
pub struct App {
    /// Search pattern
    #[clap(value_name = "PATTERN", required = true)]
    pattern: String,

    /// Input file(s)
    #[clap(value_name = "FILES", multiple = true, default_value = "-")]
    files: Vec<String>,

    /// Case insensitive
    #[clap(short = 'i', long = "insensitive", takes_value = false)]
    insensitive: bool,

    /// Recursive search
    #[clap(short = 'r', long = "recursive", takes_value = false)]
    recursive: bool,

    /// Count occurrences
    #[clap(short = 'c', long = "count", takes_value = false)]
    count: bool,

    /// Invert match
    #[clap(short = 'v', long = "invert-match", takes_value = false)]
    invert_match: bool,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        let pattern = self.build_pattern()?;
        let entries = find_files(&self.files, self.recursive);
        let entries_len = entries.len();

        let print = |filename: &str, val: &str| {
            if entries_len > 1 {
                print!("{}:{}", filename, val);
            } else {
                print!("{}", val);
            }
        };

        for entry in entries {
            match entry {
                Err(e) => {
                    eprintln!("{}", e)
                }

                Ok(filename) => match open(&filename) {
                    Err(e) => {
                        eprintln!("{}: {}", filename, e)
                    }

                    Ok(file) => match find_lines(file, &pattern, self.invert_match) {
                        Err(e) => {
                            eprintln!("{}", e)
                        }

                        Ok(matches) => {
                            if self.count {
                                print(&filename, &format!("{}\n", matches.len()));
                            } else {
                                for line in &matches {
                                    print(&filename, line);
                                }
                            }
                        }
                    },
                },
            }
        }

        Ok(())
    }

    fn build_pattern(&self) -> AppResult<Regex> {
        RegexBuilder::new(&self.pattern)
            .case_insensitive(self.insensitive)
            .build()
            .map_err(|_| {
                Self::command()
                    .error(
                        ErrorKind::InvalidValue,
                        format!("invalid pattern \"{}\"", &self.pattern),
                    )
                    .exit();
            })
    }
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_lines<T: BufRead>(
    mut file: T,
    pattern: &Regex,
    invert_match: bool,
) -> AppResult<Vec<String>> {
    let mut matches = vec![];

    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        if pattern.is_match(&line) ^ invert_match {
            matches.push(mem::take(&mut line));
        }

        line.clear();
    }

    Ok(matches)
}

fn find_files(paths: &[String], recursive: bool) -> Vec<AppResult<String>> {
    let mut results = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            let entries = WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_type().is_file());

                            for entry in entries {
                                results.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            results.push(Err(From::from(format!("{} is a directory", path))));
                        }
                    }

                    if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                    }
                }

                Err(e) => {
                    results.push(Err(From::from(format!("{}: {}", path, e))));
                }
            },
        }
    }

    results
}
