use std::env;

fn main() {
    let mut config = day_3::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problems parsing arguments: {}", err);
        std::process::exit(1);
    });

    config.first = false;

    if let Err(e) = day_3::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
