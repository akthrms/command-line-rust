use clap::Parser;
use commr::App;

fn main() {
    if let Err(e) = App::parse().run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
