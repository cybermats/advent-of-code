use std::env;
use std::fs;
use std::error::Error;
use crate::common::aggregate;
use crate::common::first::{epsilon, gamma};

pub struct Config {
    filename: String,
    pub first: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        match args.next() {
            Some(filename) => Ok(Config { filename, first: true }),
            None => Err("no arguments"),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let counts = aggregate(&content)?;

    if config.first {
        let gamma = gamma(&counts);
        let epsilon = epsilon(&counts);
        println!("Gamma: {}, Epsilon: {}, Power Consumption: {}", gamma, epsilon, gamma*epsilon);
    }

    Ok(())
}

mod common {
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct Readings<'a> {
        pub data: Vec<&'a str>,
        pub width: usize,
    }


    pub struct Counts {
        zeroes: Vec<u32>,
        ones: Vec<u32>,
        pub size: usize,
    }

    impl Counts {
        pub fn new(size: usize) -> Counts {
            Counts { zeroes: vec![0; size], ones: vec![0; size], size }
        }

        pub fn read(&mut self, line: &str) -> () {
            if line.len() != self.size {
                panic!("String is not the same size as specified in Counts.");
            }

            for (i, c) in line.char_indices() {
                match c {
                    '0' => self.zeroes[i] += 1,
                    '1' => self.ones[i] += 1,
                    _ => panic!("invalid string"),
                }
            }
        }

        pub fn most_common(&self, idx: usize) -> char {
            if self.zeroes[idx] < self.ones[idx] {
                '1'
            } else {
                '0'
            }
        }
        pub fn least_common(&self, idx: usize) -> char {
            if self.zeroes[idx] >= self.ones[idx] {
                '1'
            } else {
                '0'
            }
        }
    }

    pub fn parse_data<'a>(content: &'a str) -> Result<Readings<'a>, &'static str> {
        let data: Vec<&str> =
            content.lines().collect();

        if data.is_empty() {
            return Err("Empty content");
        }

        let width = data[0].len();
        if data.iter().any(|x| x.len() != width) {
            return Err("Invalid format, multiple line sizes.");
        }
        Ok(Readings { data, width })
    }

    pub fn aggregate(content: &str) -> Result<Counts, &'static str> {
        let readings = parse_data(content)?;
        let mut counts = Counts::new(readings.width);
        readings.data.iter().for_each(|x| counts.read(x));
        Ok(counts)
    }

    pub mod first {
        use crate::common::Counts;

        pub fn gamma(counts: &Counts) -> u32 {
            let mut result: u32 = 0;
            for i in 0..counts.size {
                result *= 2;
                if counts.most_common(i) == '1' {
                    result += 1;
                }
            }
            result
        }

        pub fn epsilon(counts: &Counts) -> u32 {
            let mut result: u32 = 0;
            for i in 0..counts.size {
                result *= 2;
                if counts.least_common(i) == '1' {
                    result += 1;
                }
            }
            result
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::common;
    use crate::common::parse_data;

    #[test]
    fn empty_content() {
        let content = "";
        assert_eq!(parse_data(content), Err("Empty content"));
    }

    #[test]
    fn different_sizes() {
        let content = "\
01
010";
        assert_eq!(common::parse_data(content), Err("Invalid format, multiple line sizes."));
    }

    #[test]
    fn can_parse_data() {
        let content = "\
01
00";
        let expected = common::Readings { data: vec!["01", "00"], width: 2 };
        assert_eq!(common::parse_data(content), Ok(expected));
    }

    #[test]
    fn aggregate_line() {
        let data = "01";
        let mut count = common::Counts::new(2);
        count.read(data);
        assert_eq!(count.most_common(0), '0');
        assert_eq!(count.least_common(0), '1');
        assert_eq!(count.most_common(1), '1');
        assert_eq!(count.least_common(1), '0');
    }

    #[test]
    fn aggregate_lines() {
        let data = vec!["001", "011"];
        let mut count = common::Counts::new(3);
        data.iter().for_each(|x| count.read(x));
        assert_eq!(count.most_common(0), '0');
        assert_eq!(count.least_common(0), '1');
        assert_eq!(count.most_common(1), '0');
        assert_eq!(count.least_common(1), '1');
        assert_eq!(count.most_common(2), '1');
        assert_eq!(count.least_common(2), '0');
    }

    #[test]
    fn aggregate_content() {
        let content = "\
001
011";
        let counts = common::aggregate(content);
        assert!(!counts.is_err());
        let counts = counts.unwrap();
        assert_eq!(counts.most_common(0), '0');
        assert_eq!(counts.least_common(0), '1');
        assert_eq!(counts.most_common(1), '0');
        assert_eq!(counts.least_common(1), '1');
        assert_eq!(counts.most_common(2), '1');
        assert_eq!(counts.least_common(2), '0');
    }

    #[test]
    fn gamma_value() {
        let data = vec!["00", "01", "10", "11"];
        for (idx, datum) in data.iter().enumerate() {
            let mut count = common::Counts::new(2);
            count.read(datum);
            let g = common::first::gamma(&count);
            assert_eq!(g, idx as u32);
        }
    }

    #[test]
    fn epsilon_value() {
        let data = vec!["11", "10", "01", "00"];
        for (idx, datum) in data.iter().enumerate() {
            let mut count = common::Counts::new(2);
            count.read(datum);
            let g = common::first::epsilon(&count);
            assert_eq!(g, idx as u32);
        }
    }
}
