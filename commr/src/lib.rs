use clap::Parser;
use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust comm"
)]
pub struct App {
    /// Input file 1
    #[clap(value_name = "FILE1", takes_value = true, required = true)]
    file1: String,

    /// Input file 2
    #[clap(value_name = "FILE2", takes_value = true, required = true)]
    file2: String,

    /// Suppress printing of column 1
    #[clap(short = '1', takes_value = false)]
    show_column1: bool,

    /// Suppress printing of column 2
    #[clap(short = '2', takes_value = false)]
    show_column2: bool,

    /// Suppress printing of column 3
    #[clap(short = '3', takes_value = false)]
    show_column3: bool,

    /// Case insensitive comparison of lines
    #[clap(short = 'i', takes_value = false)]
    insensitive: bool,

    /// Output delimiter
    #[clap(
        short = 'd',
        long = "delimiter",
        value_name = "DELIM",
        default_value = "\t",
        takes_value = true
    )]
    delimiter: String,
}

#[derive(Debug)]
enum Column<'a> {
    Column1(&'a str),
    Column2(&'a str),
    Column3(&'a str),
}

impl App {
    pub fn run(self) -> AppResult<()> {
        let file1 = &self.file1;
        let file2 = &self.file2;

        if file1 == "-" && file2 == "-" {
            return Err(From::from("both input files cannot be STDIN (\"-\")"));
        }

        let case = |line: String| -> String {
            if self.insensitive {
                line.to_lowercase()
            } else {
                line
            }
        };

        let print = |column: Column| {
            let mut columns = vec![];

            match column {
                Column::Column1(val) => {
                    if !self.show_column1 {
                        columns.push(val);
                    }
                }

                Column::Column2(val) => {
                    if !self.show_column2 {
                        if !self.show_column1 {
                            columns.push("");
                        }
                        columns.push(val);
                    }
                }

                Column::Column3(val) => {
                    if !self.show_column3 {
                        if !self.show_column1 {
                            columns.push("");
                        }
                        if !self.show_column2 {
                            columns.push("");
                        }
                        columns.push(val);
                    }
                }
            };

            if !columns.is_empty() {
                println!("{}", columns.join(&self.delimiter));
            }
        };

        let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
        let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

        let mut line1 = lines1.next();
        let mut line2 = lines2.next();

        while line1.is_some() || line2.is_some() {
            match (&line1, &line2) {
                (Some(val1), Some(val2)) => match val1.cmp(val2) {
                    Ordering::Equal => {
                        print(Column::Column3(val1));
                        line1 = lines1.next();
                        line2 = lines2.next();
                    }

                    Ordering::Less => {
                        print(Column::Column1(val1));
                        line1 = lines1.next();
                    }

                    Ordering::Greater => {
                        print(Column::Column2(val2));
                        line2 = lines2.next();
                    }
                },

                (Some(val1), None) => {
                    print(Column::Column1(val1));
                    line1 = lines1.next();
                }

                (None, Some(val2)) => {
                    print(Column::Column2(val2));
                    line2 = lines2.next();
                }

                _ => (),
            }
        }

        Ok(())
    }
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
