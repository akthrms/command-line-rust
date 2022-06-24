use clap::Parser;
use tailr::App;

fn main() {
    if let Err(e) = App::parse().run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
