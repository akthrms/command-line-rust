use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    version = "0.1.0",
    author = "akthrms <akt.hrms@gmail.com>",
    about = "Rust echo"
)]
struct Config {
    /// Input text
    #[clap(value_name = "TEXT", required = true, min_values = 1)]
    text: Vec<String>,

    /// Do not print newline
    #[clap(short = 'n', takes_value = false)]
    omit_newline: bool,
}

fn main() {
    let Config { text, omit_newline } = Config::parse();
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
