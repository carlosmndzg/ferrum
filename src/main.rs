use std::env;

use ferrum::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    if let Err(err) = ferrum::run(config) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
