use day_4::run;
use std::env;

fn main() {
    let mut args = env::args();
    let filename = match args.nth(1) {
        None => {
            eprintln!("usage: day_4 FILE");
            std::process::exit(1);
        }
        Some(val) => val,
    };
    if let Err(err) = run(&filename) {
        eprintln!("Application Error: {}", err);
        std::process::exit(1);
    }
}
