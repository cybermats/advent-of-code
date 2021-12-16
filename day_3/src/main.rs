use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let config = day_3::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("usage: day_3 FILE");
        eprintln!("{}", err);
        std::process::exit(1);
    });



    let filename = match std::env::args().nth(1) {
        Some(val) => val,
        None => {
            eprintln!("usage: {} FILE", std::env::args().next().unwrap());
            std::process::exit(-1);
        }
    };

    let file = match File::open(&filename) {
        Ok(f) => BufReader::new(f),
        Err(e) => {
            eprintln!("unable to read file {}: {}", &filename, e);
            std::process::exit(-1);
        }
    };


    let (data, width) = read_data(file);

    if data.is_empty() || data.first().is_none() {
        eprintln!("empty file.");
        std::process::exit(-1);
    }

    calc_gamma_epsilon(data, width);
}

fn calc_gamma_epsilon(data: Vec<String>, width: usize) {
    let mut ones: Vec<u32> = Vec::new();
    ones.resize(width, 0);
    let mut zeroes: Vec<u32> = Vec::new();
    zeroes.resize(width, 0);

    if ones.len() != zeroes.len() {
        panic!("Inconsistent state.");
    }


    for datum in data {
        for (i, c) in datum.char_indices() {
            match c {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => panic!("Invalid file format."),
            };
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..ones.len() {
        gamma *= 2;
        epsilon *= 2;
        if ones[i] > zeroes[i] {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("Gamma: {}, Epsilon: {}, Power Consumption: {}", gamma, epsilon, gamma * epsilon);
}

fn read_data(file: BufReader<File>) -> (Vec<String>, usize) {
    let mut data = Vec::new();

    let mut width = usize::MAX;
    for line in file.lines() {
        let line = line.unwrap();
        if width == usize::MAX {
            width = line.len();
        }
        if line.len() != width {
            panic!("Not all rows have the same length.");
        }
        data.push(line);
    }
    (data, width)
}
