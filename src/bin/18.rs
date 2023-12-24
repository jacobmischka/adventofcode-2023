use std::{collections::HashSet, fmt::Display, io};

use adventofcode_2023::grid::{Direction, Grid, Position, SignedPosition};

fn main() {
    let mut true_instructions: Vec<Instruction> = Vec::new();
    let instructions: Vec<Instruction> = io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                let mut pieces = line.split_ascii_whitespace();
                let direction = match pieces.next().unwrap() {
                    "U" => Direction::North,
                    "D" => Direction::South,
                    "L" => Direction::West,
                    "R" => Direction::East,
                    d => panic!("invalid direction {d}"),
                };
                let dist: usize = pieces.next().unwrap().parse().unwrap();
                let hex = pieces.next().unwrap().replace(&['(', ')'], "");
                let instruction = Instruction { direction, dist };

                let true_instruction = Instruction::from_hex(&hex).unwrap();
                true_instructions.push(true_instruction);

                Some(instruction)
            }
        })
        .collect();

    let part1 = excavate(&instructions).unwrap();
    println!("Part 1: {part1}");

    let part2 = excavate(&true_instructions).unwrap();
    println!("Part 2: {part2}");
}

fn excavate(instructions: &[Instruction]) -> Option<usize> {
    let mut stack: Vec<SignedPosition> = Vec::new();
    let mut pos = SignedPosition(0, 0);
    for instruction in instructions {
        //
    }

    Some(0)
}

struct Instruction {
    direction: Direction,
    dist: usize,
}

impl Instruction {
    fn from_hex(hex: &str) -> Result<Self, String> {
        if !hex.starts_with('#') {
            return Err(format!("invalid hex {hex} missing #"));
        }

        let dist = usize::from_str_radix(&hex[1..hex.len() - 1], 16)
            .map_err(|_| format!("invalid hex distance {hex}"))?;
        let direction_num: u8 = hex[&hex.len() - 1..]
            .parse()
            .map_err(|_| format!("invalid hex direction {hex}"))?;
        let direction = match direction_num {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => {
                return Err(format!("invalid hex direction {hex}"));
            }
        };

        Ok(Instruction { direction, dist })
    }
}
