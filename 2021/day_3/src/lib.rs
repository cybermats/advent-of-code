use crate::common::aggregate;
use crate::common::first::{epsilon, gamma};
use crate::common::second::{co2, oxygen};
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
    pub first: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        match args.next() {
            Some(filename) => Ok(Config {
                filename,
                first: true,
            }),
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
        println!(
            "Gamma: {}, Epsilon: {}, Power Consumption: {}",
            gamma,
            epsilon,
            gamma * epsilon
        );
    } else {
        let readings = common::parse_data(&content)?;
        let oxygen = oxygen(&readings);
        let co2 = co2(&readings);
        println!(
            "Oxygen: {}, CO2: {}, Life support: {}",
            oxygen,
            co2,
            oxygen * co2
        );
    }

    Ok(())
}

mod common {
    use std::cmp::Ordering;

    #[derive(PartialEq, Debug)]
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
            Counts {
                zeroes: vec![0; size],
                ones: vec![0; size],
                size,
            }
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
            match self.zeroes[idx].cmp(&self.ones[idx]) {
                Ordering::Less => '1',
                Ordering::Equal => '1',
                Ordering::Greater => '0',
            }
        }
        pub fn least_common(&self, idx: usize) -> char {
            match self.zeroes[idx].cmp(&self.ones[idx]) {
                Ordering::Less => '0',
                Ordering::Equal => '0',
                Ordering::Greater => '1',
            }
        }
    }

    pub fn parse_data<'a>(content: &'a str) -> Result<Readings<'a>, &'static str> {
        let data: Vec<&str> = content.lines().collect();

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

    pub fn filter_bit<'a>(
        data: &Vec<&'a str>,
        position: usize,
        filter_value: char,
    ) -> Vec<&'a str> {
        let i = data.iter();
        let f = i.filter(|e| e.chars().nth(position).unwrap() == filter_value);
        let c: Vec<&str> = f.cloned().collect();
        c
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

    pub mod second {
        use crate::common::{filter_bit, Counts, Readings};

        pub fn oxygen(readings: &Readings) -> u32 {
            let mut data = readings.data.clone();
            for pos in 0..readings.width {
                let mut counts = Counts::new(readings.width);
                data.iter().for_each(|d| counts.read(d));
                data = filter_bit(&data, pos, counts.most_common(pos));
                if data.len() == 1 {
                    break;
                }
            }
            let item = data[0];
            let mut oxygen = 0;
            for c in item.chars() {
                oxygen *= 2;
                if c == '1' {
                    oxygen += 1;
                }
            }
            oxygen
        }

        pub fn co2(readings: &Readings) -> u32 {
            let mut data = readings.data.clone();
            for pos in 0..readings.width {
                let mut counts = Counts::new(readings.width);
                data.iter().for_each(|d| counts.read(d));
                data = filter_bit(&data, pos, counts.least_common(pos));
                if data.len() == 1 {
                    break;
                }
            }
            let item = data[0];
            let mut oxygen = 0;
            for c in item.chars() {
                oxygen *= 2;
                if c == '1' {
                    oxygen += 1;
                }
            }
            oxygen
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
        assert_eq!(
            common::parse_data(content),
            Err("Invalid format, multiple line sizes.")
        );
    }

    #[test]
    fn can_parse_data() {
        let content = "\
01
00";
        let expected = common::Readings {
            data: vec!["01", "00"],
            width: 2,
        };
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
    fn aggregate_content() {
        let content = "\
001
011";
        let counts = common::aggregate(content);
        assert!(!counts.is_err());
        let counts = counts.unwrap();
        assert_eq!(counts.most_common(0), '0');
        assert_eq!(counts.least_common(0), '1');

        assert_eq!(counts.most_common(1), '1');
        assert_eq!(counts.least_common(1), '0');

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

    #[test]
    fn filter_bit_criteria() {
        let data = vec!["11", "10", "01", "00"];
        let filtered = common::filter_bit(&data, 0, '0');
        assert_eq!(filtered, vec!["01", "00"]);
        let filtered = common::filter_bit(&data, 0, '1');
        assert_eq!(filtered, vec!["11", "10"]);
        let filtered = common::filter_bit(&data, 1, '0');
        assert_eq!(filtered, vec!["10", "00"]);
        let filtered = common::filter_bit(&data, 1, '1');
        assert_eq!(filtered, vec!["11", "01"]);
    }

    #[test]
    fn oxygen_value() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let readings = common::Readings { data, width: 5 };

        let oxygen = common::second::oxygen(&readings);
        assert_eq!(oxygen, 23);
    }

    #[test]
    fn co2_value() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let readings = common::Readings { data, width: 5 };

        let co2 = common::second::co2(&readings);
        assert_eq!(co2, 10);
    }
}
