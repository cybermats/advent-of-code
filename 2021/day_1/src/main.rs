use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Config {
    filename: Option<String>,
    windowing: bool,
}

fn parse_config() -> Option<Config> {
    let mut args = env::args();
    args.next();
    let config = match args.next() {
        Some(val) => match val.as_str() {
            "-w" => Config {
                filename: None,
                windowing: true,
            },
            other => match other.starts_with("-") {
                false => Config {
                    filename: Some(String::from(other)),
                    windowing: false,
                },
                true => return None,
            },
        },
        None => return None,
    };

    match args.next() {
        Some(val) => {
            if config.filename.is_some() {
                None
            } else {
                Some(Config {
                    filename: Some(val),
                    windowing: config.windowing,
                })
            }
        }
        None => {
            if config.filename.is_some() {
                Some(config)
            } else {
                None
            }
        }
    }
}

fn usage() {
    eprintln!("usage: day_1 FILENAME");
}

fn raw_extract(reader: BufReader<File>) -> u32 {
    let mut last = u32::MAX;
    let mut increase_counter = 0;
    for line in reader.lines() {
        let number: u32 = line.unwrap().parse().expect("Invalid number in file.");
        let rate = match number.cmp(&last) {
            Ordering::Less => "decrease",
            Ordering::Equal => "same",
            Ordering::Greater => {
                increase_counter = increase_counter + 1;
                "increase"
            }
        };
        println!("{} ({})", number, rate);
        last = number;
    }
    increase_counter
}

fn windowed_extract(reader: BufReader<File>) -> u32 {
    let mut sums = [0, 0, 0];
    let mut idx: usize = 0;
    let mut last = u32::MAX;
    let mut increase_counter = 0;
    let mut skip_rows = 2;
    for line in reader.lines() {
        let number: u32 = line.unwrap().parse().expect("Invalid number in file.");
        for i in 0..3 {
            sums[i] += number;
        }
        if sums[idx] > last {
            increase_counter = increase_counter + 1;
        }
        if skip_rows == 0 {
            last = sums[idx];
        } else {
            skip_rows -= 1;
        }
        sums[idx] = 0;
        idx = (idx + 1) % 3;
    }
    increase_counter
}

fn main() {
    let config = parse_config();
    let buf_reader = match &config {
        Some(val) => match &val.filename {
            Some(filename) => match File::open(filename) {
                Ok(file) => BufReader::new(file),
                Err(e) => {
                    eprintln!("Unable to open file: {}", e);
                    std::process::exit(-1);
                }
            },
            None => {
                usage();
                std::process::exit(-1);
            }
        },
        None => {
            usage();
            std::process::exit(-1);
        }
    };

    let increase_counter = match config.unwrap().windowing {
        false => raw_extract(buf_reader),
        true => windowed_extract(buf_reader),
    };
    println!("Total increases: {}", increase_counter);
}
