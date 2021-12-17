pub mod bingo_card;

pub mod numbers {
    use std::str::Lines;

    pub fn parse_numbers(input: &mut Lines) -> Result<Vec<u32>, &'static str> {
        let mut result = vec![];
        for item in input.next().expect("invalid format").split(",") {
            let val = item.parse::<u32>().expect("invalid format");
            result.push(val)
        }
        Ok(result)
    }
}
