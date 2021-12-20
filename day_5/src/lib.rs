use regex::Regex;
use std::error::Error;
use std::{cmp, fs};

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let vent_lines = parse_vents(content);
    let diagram = produce_field(vent_lines);
    println!("Points: {}", diagram.point());
    Ok(())
}

#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Vent {
    pub a: Point,
    pub b: Point,
}

impl Vent {
    pub fn new(a: Point, b: Point) -> Vent {
        Vent { a, b }
    }
}

fn parse_vents(lines: String) -> Vec<Vent> {
    let line_pattern: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    lines
        .lines()
        .map(|l| parse_vent(l, &line_pattern))
        .collect()
}

fn parse_vent(line: &str, pattern: &Regex) -> Vent {
    let values = pattern.captures(line).unwrap();
    let a = Point::new(values[1].parse().unwrap(), values[2].parse().unwrap());
    let b = Point::new(values[3].parse().unwrap(), values[4].parse().unwrap());
    Vent::new(a, b)
}

#[derive(Debug)]
struct FieldRow {
    pub row: Vec<i32>,
}

impl FieldRow {
    pub fn new(width: usize) -> FieldRow {
        FieldRow {
            row: vec![0; width],
        }
    }

    pub fn resize(&mut self, width: usize) {
        assert!(self.row.len() < width);
        self.row.resize(width, 0)
    }

    pub fn add(&mut self, col: usize) {
        self.row[col] += 1;
    }

    pub fn points(&self) -> usize {
        let overlaps = self.row.iter().filter(|&f| *f > 1);
        overlaps.count()
    }
}

#[derive(Debug)]
struct Field {
    height: usize,
    width: usize,
    field: Vec<FieldRow>,
}

impl Field {
    pub fn new() -> Field {
        Field {
            height: 0,
            width: 0,
            field: vec![],
        }
    }
    pub fn add(&mut self, vent: &Vent) {
        let largest_x = (cmp::max(vent.a.x, vent.b.x) + 1) as usize;
        let largest_y = (cmp::max(vent.a.y, vent.b.y) + 1) as usize;

        if largest_x >= self.width {
            self.field.iter_mut().for_each(|f| f.resize(largest_x));
            self.width = largest_x;
        }
        if largest_y >= self.height {
            self.field
                .resize_with((largest_y) as usize, || FieldRow::new(self.width));
            self.height = largest_y;
        }

        let smallest_x = cmp::min(vent.a.x, vent.b.x) as usize;
        let smallest_y = cmp::min(vent.a.y, vent.b.y) as usize;

        for row in smallest_y..largest_y {
            for col in smallest_x..largest_x {
                self.field[row].add(col);
            }
        }
    }

    pub fn point(&self) -> usize {
        self.field.iter().map(|f| f.points()).sum()
    }
}

fn produce_field(vents: Vec<Vent>) -> Field {
    let mut field = Field::new();
    let vents = vents.iter().filter(|v| v.a.x == v.b.x || v.a.y == v.b.y);
    for vent in vents {
        field.add(&vent);
    }
    field
}
