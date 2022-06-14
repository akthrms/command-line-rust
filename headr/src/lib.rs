use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust head"
)]
pub struct App {
    /// Input file(s)
    #[clap(value_name = "FILE", multiple_occurrences = true, default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[clap(
        short = 'n',
        long = "lines",
        value_name = "LINES",
        default_value_t = 10,
        parse(try_from_str = parse_lines)
    )]
    lines: usize,

    /// Number of bytes
    #[clap(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        takes_value = true,
        conflicts_with = "lines",
        parse(try_from_str = parse_bytes)
    )]
    bytes: Option<usize>,
}

impl App {
    pub fn run(self) -> AppResult<()> {
        for (file_num, filename) in self.files.iter().enumerate() {
            match open(&filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(mut file) => {
                    if self.files.len() > 1 {
                        println!(
                            "{}==> {} <==",
                            if file_num > 0 { "\n" } else { "" },
                            filename
                        );
                    }

                    if let Some(bytes) = self.bytes {
                        let mut handle = file.take(bytes as u64);
                        let mut buffer = vec![0; bytes];
                        let bytes = handle.read(&mut buffer)?;
                        print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
                    } else {
                        let mut line = String::new();
                        for _ in 0..self.lines {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_lines(val: &str) -> Result<usize, String> {
    parse_positive_int(val).map_err(|e| format!("illegal line count -- {}", e))
}

fn parse_bytes(val: &str) -> Result<usize, String> {
    parse_positive_int(val).map_err(|e| format!("illegal byte count -- {}", e))
}

fn parse_positive_int(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo");

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0");
}
