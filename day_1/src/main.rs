use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.get(1).is_none() {
        println!("usage: day_1 FILENAME");
        std::process::exit(-1);
    }

    let filename = &args[1];

    let f = File::open(filename);
    let f = match f {
        Ok(file) => file,
        Err(e) => {
            println!("Unable to open file: {}", e);
            std::process::exit(-1);
        }
    };

    let buf_reader = BufReader::new(f);

    let mut last = std::u32::MAX;
    let mut increase_counter = 0;
    for line in buf_reader.lines() {
        let number: u32 = line.unwrap().parse().unwrap();
        let rate = if number < last {
            "decrease"
        } else if number > last {
            increase_counter = increase_counter + 1;
            "increase"
        } else {
            "same"
        };
        println!("{} ({})", number, rate);
        last = number;
    }

    println!("Total increases: {}", increase_counter);
}
