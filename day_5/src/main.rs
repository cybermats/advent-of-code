fn main() {
    let mut args = std::env::args();
    let filename = match args.nth(1) {
        None => {
            eprintln!("usage: day_5 FILE");
            std::process::exit(1);
        },
        Some(val) => val
    };
    if let Err(err) = day_5::run(&filename) {
        eprintln!("application error: {}", err);
        std::process::exit(1);
    }
}
