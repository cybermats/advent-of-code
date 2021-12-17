mod common;

use crate::common::bingo_card::{BingoCard, BingoCards};
use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let mut lines = content.lines();
    let numbers = common::numbers::parse_numbers(&mut lines)?;

    let mut cards =
        BingoCards::new(&mut lines).collect::<Result<Vec<BingoCard>, &'static str>>()?;

    for number in numbers {
        println!("Drawn number: {}", number);
        for card in cards.iter_mut() {
            if card.mark(number) {
                println!(
                    "Score: {}, Final Score: {}, Board:\n{}",
                    card.score(),
                    card.score() * number,
                    card
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::bingo_card::BingoCard;

    #[test]
    fn create_two_bingo_correct_and_not_another() {
        let card = "
 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24

0 1 2 3 4
5 6 7 8 9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24";

        let mut lines = card.lines();
        let result: Result<Vec<BingoCard>, &'static str> =
            common::bingo_card::BingoCards::new(&mut lines).collect();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn mark_rows() {
        let card = "\
 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24";

        for start in (0..25).step_by(5) {
            let mut card = common::bingo_card::BingoCard::new(&mut card.lines()).unwrap();
            for number in start..(start + 4) {
                assert!(!card.mark(number));
            }
            assert!(card.mark(start + 4));
        }
    }

    #[test]
    fn mark_columns() {
        let card = "\
 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24";

        for start in 0..5 {
            let mut card = common::bingo_card::BingoCard::new(&mut card.lines()).unwrap();
            for number in (start..20).step_by(5) {
                assert!(!card.mark(number));
            }
            assert!(card.mark(start + 20));
        }
    }
    /********
    * Apparently diagonals doesn't count
    *********
        #[test]
        fn mark_diagonal() {
            let input = "\
     0  1  2  3  4
     5  6  7  8  9
    10 11 12 13 14
    15 16 17 18 19
    20 21 22 23 24";

            let mut card = common::bingo_card::BingoCard::new(&mut input.lines()).unwrap();
            for number in (0..20).step_by(6) {
                assert!(!card.mark(number));
            }
            assert!(card.mark(24));

            let mut card = common::bingo_card::BingoCard::new(&mut input.lines()).unwrap();
            for number in (4..20).step_by(4) {
                assert!(!card.mark(number));
            }
            assert!(card.mark(20));
        }
    */
    #[test]
    fn score() {
        let input = "\
 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24";

        let mut card = common::bingo_card::BingoCard::new(&mut input.lines()).unwrap();

        assert_eq!(24 * 25 / 2, card.score());
        card.mark(12);
        assert_eq!(24 * 25 / 2 - 12, card.score());
        card.mark(10);
        assert_eq!(24 * 25 / 2 - 12 - 10, card.score());
        card.mark(30);
        assert_eq!(24 * 25 / 2 - 12 - 10, card.score());
    }

    #[test]
    fn invalid_board_empty_line() {
        let card = "\
0 1 2 3 4
5 6 7 8 9

10 11 12 13 14
15 16 17 18 19
20 21 22 23 24";
        let result = common::bingo_card::BingoCard::new(&mut card.lines());
        assert_eq!(result.err(), Some("invalid format, missing number"));
    }

    #[test]
    fn invalid_board_empty_string() {
        let card = "";
        let result = common::bingo_card::BingoCard::new(&mut card.lines());
        assert_eq!(result.err(), Some("invalid format, lack of data"));
    }

    #[test]
    fn invalid_board_missing_number() {
        let card = "\
0 1 2 3 4
5 6 7 8 9
10 11 12 13 14
15 16 17 18 19
20 21 22 23

";
        let result = common::bingo_card::BingoCard::new(&mut card.lines());
        assert_eq!(result.err(), Some("invalid format, missing number"));
    }

    #[test]
    fn fail_to_create_multiple_cards() {
        let card = "
 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24

0 1 2 3 4
5 6 7 8 9
10 11 12 13
15 16 17 18 19
20 21 22 23 24";

        let mut lines = card.lines();
        let result: Result<Vec<BingoCard>, &'static str> =
            common::bingo_card::BingoCards::new(&mut lines).collect();

        assert!(result.is_err());
    }
}
