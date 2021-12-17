use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::Lines;

pub struct BingoCards<'a> {
    input: &'a mut Lines<'a>,
}

impl<'a> BingoCards<'a> {
    pub fn new(input: &'a mut Lines<'a>) -> BingoCards<'a> {
        BingoCards { input }
    }
}

impl<'a> Iterator for BingoCards<'a> {
    type Item = Result<BingoCard, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            None => None,
            Some(line) => {
                if line.len() == 0 {
                    Some(BingoCard::new(self.input))
                } else {
                    Some(Err("invalid format, missing delimiter"))
                }
            }
        }
    }
}

pub struct BingoCard {
    marks: Vec<bool>,
    numbers: HashMap<u32, usize>,
    live: bool,
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut order = vec![0; 25];
        self.numbers.iter().for_each(|(n, i)| {
            order[*i] = *n;
        });

        for r in 0..5 {
            for c in 0..5 {
                let i = r * 5 + c;
                if self.marks[i] {
                    write!(f, "[{:2}] ", order[i]).expect("unable to print");
                } else {
                    write!(f, " {:2}  ", order[i]).expect("unable to print");
                }
            }
            write!(f, "\n").expect("unable to print");
        }
        Ok(())
    }
}

impl BingoCard {
    pub fn new(card: &mut Lines) -> Result<BingoCard, &'static str> {
        let mut numbers: HashMap<u32, usize> = HashMap::new();
        let marks = vec![false; 25];
        for row_index in 0..5 {
            let line = match card.next() {
                Some(val) => val,
                None => return Err("invalid format, lack of data"),
            };
            let mut row = vec![];
            for token in line.split_whitespace() {
                let number = match token.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => return Err("invalid format, not a number"),
                };
                row.push(number);
            }
            if row.len() != 5 {
                return Err("invalid format, missing number");
            }
            let row_indices = (row_index * 5)..(row_index * 5 + 5);
            let pairs: Vec<(u32, usize)> = row.into_iter().zip(row_indices).collect();
            numbers.extend(pairs);
        }
        Ok(BingoCard {
            marks,
            numbers,
            live: true,
        })
    }

    pub fn mark(&mut self, number: u32) -> bool {
        if !self.live {
            return false;
        }

        let idx = match self.numbers.get(&number) {
            None => return false,
            Some(val) => val.clone(),
        };
        self.marks[idx] = true;

        if self.check() {
            self.live = false;
            true
        } else {
            false
        }
    }

    pub fn score(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|(_, v)| !self.marks[**v])
            .map(|(k, _)| k)
            .sum()
    }

    fn check(&self) -> bool {
        let mut rows = [true; 5];
        let mut cols = [true; 5];
        for i in 0..25 {
            rows[i / 5] &= self.marks[i];
            cols[i % 5] &= self.marks[i];
        }

        // Check rows
        if rows.iter().fold(false, |a, b| a || *b) {
            return true;
        }

        // Check columns
        if cols.iter().fold(false, |a, b| a || *b) {
            return true;
        }
        /*
                // Check diagonals
                if self.marks[0] && self.marks[6] && self.marks[12] && self.marks[18] && self.marks[24] {
                    return true;
                }
                if self.marks[4] && self.marks[8] && self.marks[12] && self.marks[16] && self.marks[20] {
                    return true;
                }
        */
        false
    }
}
