use std::collections::HashMap;

pub fn run(initial_state: Vec<u8>, runs: u32) {
    let mut school = School::new(initial_state);
    for _ in 0..runs {
        school.age();
    }
    println!("After {} runs, we have {} fishes.", runs, school.count());
}


struct School {
    generations: HashMap<u8, u64>,
}

impl School {
    pub(crate) fn new(initial_state: Vec<u8>) -> School {
        let mut map = HashMap::new();
        for age in initial_state {
            map.entry(age)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        School{generations: map}
    }


    pub(crate) fn count(&self) -> u64 {
        self.generations.values()
            .fold(0, |acc, e| acc + e)
    }

    pub(crate) fn age(&mut self) {
        let mut map = HashMap::new();
        for (&age, &count) in self.generations.iter() {
            if age == 0 {
                map.entry(6)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
                map.insert(8, count);
            } else {
                map.entry(age - 1)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        }
        self.generations = map;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_school() {
        let school = School::new(vec![3,4,3,1,2]);
        assert_eq!(school.count(), 5);
    }

    #[test]
    fn age() {
        let mut school = School::new(vec![3,4,3,1,2]);
        school.age();
        assert_eq!(school.count(), 5);
        school.age();
        assert_eq!(school.count(), 6);
        school.age();
        assert_eq!(school.count(), 7);
        school.age();
        assert_eq!(school.count(), 9);
    }

    #[test]
    fn part_1_test() {
        let mut school = School::new(vec![3,4,3,1,2]);
        for _ in 0..18 {
            school.age();
        }
        assert_eq!(school.count(), 26);
    }

    #[test]
    fn part_1() {
        let mut school = School::new(vec![3,4,3,1,2]);
        for _ in 0..80 {
            school.age();
        }
        assert_eq!(school.count(), 5934);
    }

}