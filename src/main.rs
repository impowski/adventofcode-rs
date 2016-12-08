use std::io::{self, Read};
use std::fs::File;

#[derive(Debug)]
enum Compas {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn x(&mut self, x: i32) -> &mut Self {
        self.x += x;
        self
    }

    fn y(&mut self, y: i32) -> &mut Self {
        self.y += y;
        self
    }

    fn set(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {
    let input = read_input_from_file("input.txt").map(|s| s).unwrap();
    let instructions: Vec<_> = input.trim().split(", ").collect();
    let landed = Position::new();
    let mut compas = Compas::North;
    let mut headquartes = Position::new();
    for ins in instructions {
        let direction = &ins[0..1];
        let steps = ins[1..].parse::<i32>().map(|n| n).unwrap();
        match compas {
            Compas::North => {
                if direction == "L" {
                    headquartes.x(-steps).set();
                    compas = Compas::West;
                } else {
                    headquartes.x(steps).set();
                    compas = Compas::East;
                }
            }
            Compas::South => {
                if direction == "L" {
                    headquartes.x(steps).set();
                    compas = Compas::East;
                } else {
                    headquartes.x(-steps).set();
                    compas = Compas::West;
                }
            }
            Compas::East => {
                if direction == "L" {
                    headquartes.y(steps).set();
                    compas = Compas::North;
                } else {
                    headquartes.y(-steps).set();
                    compas = Compas::South;
                }
            }
            Compas::West => {
                if direction == "L" {
                    headquartes.y(-steps).set();
                    compas = Compas::South;
                } else {
                    headquartes.y(steps).set();
                    compas = Compas::North;
                }
            }
        }
    }
    println!("{:?}", ((landed.x - headquartes.x).abs() + (landed.y - headquartes.y).abs()));
}

fn read_input_from_file(name: &str) -> Result<String, io::Error> {
    let mut input_file = File::open(name)?;
    let mut data = String::new();

    input_file.read_to_string(&mut data)?;

    Ok(data)
}
