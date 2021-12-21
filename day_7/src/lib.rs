fn geo_sum(val: u32, pos: u32) -> u32 {
    let dis = (val as i32 - pos as i32).abs() as u32;
    (dis + 1) * dis / 2
}

pub fn run(initial_state: Vec<u32>) {
    assert!(!initial_state.is_empty());
    let left = *initial_state.iter().min().expect("vector can't be empty.");
    let right = *initial_state.iter().max().expect("vector can't be empty.");

    let mut min_fuel = u32::MAX;
    let mut min_position = left;
    for pos in left..right {
        let fuel = initial_state.iter().map(|&e| geo_sum(e, pos)).sum();
        if fuel < min_fuel {
            min_fuel = fuel;
            min_position = pos;
        }
    }
    println!(
        "Min fuel consumption is {} by aligning at {}",
        min_fuel, min_position
    );
}
