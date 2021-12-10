use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn new(line: String) -> Command {
        let mut words = line.split(" ");
        let command = words.next().expect("Malformed file, expected two words, got none.");
        let distance = words.next().expect("Malformed file, expected two words, got only one.");
        let distance = distance.parse().expect("Number is malformed.");
        match command {
            "forward" => Command::Forward(distance),
            "down" => Command::Down(distance),
            "up" => Command::Up(distance),
            _ => panic!("Unknown command"),
        }
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("usage: {} FILE", args[0]);
        std::process::exit(-1);
    }
    let filename = args[1].as_str();
    let f = File::open(filename)
        .expect("Something went wrong reading the file");
    let content = std::io::BufReader::new(f);

    let mut position = Position{horizontal: 0, vertical: 0, aim: 0};
    const version_1: bool = false;

    for line in content.lines() {
        let cmd = Command::new(line.expect("Unable to read file."));
        println!("{:?}", cmd);
        if version_1 {
            match cmd {
                Command::Forward(val) => position.horizontal += val,
                Command::Down(val) => position.vertical += val,
                Command::Up(val) => position.vertical -= val,
            }
        } else {
            match cmd {
                Command::Up(val) => position.aim -= val,
                Command::Down(val) => position.aim += val,
                Command::Forward(val) => {
                    position.horizontal += val;
                    position.vertical += position.aim * val;
                },

            }
        }
    }

    println!("Finished: {:?}, Product: {}", position, position.vertical * position.horizontal);
}
