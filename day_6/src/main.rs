fn main() {
    let fishes = vec![3, 4, 3, 1, 2];
    /*
    let fishes = vec![
        3, 4, 1, 1, 5, 1, 3, 1, 1, 3, 5, 1, 1, 5, 3, 2, 4, 2, 2, 2, 1, 1, 1, 1, 5, 1, 1, 1, 1, 1,
        3, 1, 1, 5, 4, 1, 1, 1, 4, 1, 1, 1, 1, 2, 3, 2, 5, 1, 5, 1, 2, 1, 1, 1, 4, 1, 1, 1, 1, 3,
        1, 1, 3, 1, 1, 1, 1, 1, 1, 2, 3, 4, 2, 1, 3, 1, 1, 2, 1, 1, 2, 1, 5, 2, 1, 1, 1, 1, 1, 1,
        4, 1, 1, 1, 1, 5, 1, 4, 1, 1, 1, 3, 3, 1, 3, 1, 3, 1, 4, 1, 1, 1, 1, 1, 4, 5, 1, 1, 3, 2,
        2, 5, 5, 4, 3, 1, 2, 1, 1, 1, 4, 1, 3, 4, 1, 1, 1, 1, 2, 1, 1, 3, 2, 1, 1, 1, 1, 1, 4, 1,
        1, 1, 4, 4, 5, 2, 1, 1, 1, 1, 1, 2, 4, 2, 1, 1, 1, 2, 1, 1, 2, 1, 5, 1, 5, 2, 5, 5, 1, 1,
        3, 1, 4, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 4, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 5, 1, 1, 3, 5,
        1, 1, 5, 5, 3, 5, 3, 4, 1, 1, 1, 3, 1, 1, 3, 1, 1, 1, 1, 1, 1, 5, 1, 3, 1, 5, 1, 1, 4, 1,
        3, 1, 1, 1, 2, 1, 1, 1, 2, 1, 5, 1, 1, 1, 1, 4, 1, 3, 2, 3, 4, 1, 3, 5, 3, 4, 1, 4, 4, 4,
        1, 3, 2, 4, 1, 4, 1, 1, 2, 1, 3, 1, 5, 5, 1, 5, 1, 1, 1, 5, 2, 1, 2, 3, 1, 4, 3, 3, 4, 3,
    ];
    */

    let gens = 256;
    let c = generations(fishes, gens);
    println!("After {} generations there are {} fishes.", gens, c);
}

fn generations(fishes: Vec<u32>, generations: u32) -> usize {
    let mut f = fishes;
    for _ in 0..generations {
        f = step(f);
    }
    f.len()
}

fn step(fishes: Vec<u32>) -> Vec<u32> {
    let mut next = age(&fishes);
    let children = children(&fishes);
    next.extend(children.iter());
    next
}

fn age(fishes: &Vec<u32>) -> Vec<u32> {
    fishes
        .iter()
        .map(|f| if *f > 0 { *f - 1 } else { 6 })
        .collect()
}

fn children(fishes: &Vec<u32>) -> Vec<u32> {
    fishes.iter().filter(|&f| *f == 0).map(|_| 8).collect()
}
