fn main() {
    let mut args = std::env::args();
    let filename = match args.nth(1) {
        None => {
            eprintln!("usage: day_5 FILE");
            std::process::exit(1);
        }
        Some(val) => val,
    };
    let part2 = true;
    if let Err(err) = day_5::run(&filename, part2) {
        eprintln!("application error: {}", err);
        std::process::exit(1);
    }
}
