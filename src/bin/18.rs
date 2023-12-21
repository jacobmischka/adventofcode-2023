use std::{collections::HashSet, fmt::Display, io};

use adventofcode_2023::grid::{Direction, Grid, Position, SignedPosition};

fn main() {
    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;

    let mut true_min_x: isize = 0;
    let mut true_max_x: isize = 0;
    let mut true_min_y: isize = 0;
    let mut true_max_y: isize = 0;

    let mut true_pos = SignedPosition(0, 0);
    let mut true_instructions: Vec<Instruction> = Vec::new();
    let mut pos = SignedPosition(0, 0);
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

                let true_instruction = Instruction::from_hex(&hex).unwrap();
                true_pos = true_pos
                    + true_instruction.direction.unit_vector() * true_instruction.dist as isize;
                true_min_x = true_min_x.min(true_pos.0);
                true_max_x = true_max_x.max(true_pos.0);
                true_min_y = true_min_y.min(true_pos.1);
                true_max_y = true_max_y.max(true_pos.1);
                true_instructions.push(true_instruction);

                pos = pos + direction.unit_vector() * dist as isize;
                min_x = min_x.min(pos.0);
                max_x = max_x.max(pos.0);
                min_y = min_y.min(pos.1);
                max_y = max_y.max(pos.1);

                Some(Instruction { direction, dist })
            }
        })
        .collect();

    let mut grid: Grid<Terrain> = Grid(vec![
        vec![
            // We flood to fill the exterior level ground
            Terrain::Interior;
            (min_x.abs() + max_x.abs()) as usize + 1
        ];
        (min_y.abs() + max_y.abs()) as usize + 1
    ]);
    let pos = Position(min_x.abs() as usize, min_y.abs() as usize);
    let part1 = excavate(&mut grid, pos, &instructions);

    println!("Part 1: {part1}");

    let mut grid: Grid<Terrain> = Grid(vec![
        vec![
            // We flood to fill the exterior level ground
            Terrain::Interior;
            (true_min_x.abs() + true_max_x.abs()) as usize + 1
        ];
        (true_min_y.abs() + true_max_y.abs()) as usize + 1
    ]);
    let pos = Position(true_min_x.abs() as usize, true_min_y.abs() as usize);
    let part2 = excavate(&mut grid, pos, &true_instructions);
    println!("Part 2: {part2}");
}

fn excavate(grid: &mut Grid<Terrain>, start: Position, instructions: &[Instruction]) -> usize {
    let mut pos = start;
    for instruction in instructions {
        let unit_vector = instruction.direction.unit_vector();
        for _ in 0..instruction.dist {
            pos = (pos + unit_vector).unwrap();
            grid[pos.1][pos.0] = Terrain::Edge;
        }
    }

    let mut leveled: HashSet<Position> = HashSet::new();

    for y in 0..grid.len() {
        if y == 0 || y == grid.len() - 1 {
            for x in 0..grid[y].len() {
                flood(grid, Position(x, y), &mut leveled);
            }
        } else {
            flood(grid, Position(0, y), &mut leveled);
            let end = grid[y].len() - 1;
            flood(grid, Position(end, y), &mut leveled);
        }
    }

    (grid.len() * grid[0].len()) - leveled.len()
}

fn flood(grid: &mut Grid<Terrain>, pos: Position, visited: &mut HashSet<Position>) {
    if visited.contains(&pos) {
        return;
    }

    match grid.get_pos_mut(pos) {
        None | Some(Terrain::Edge) => {
            return;
        }
        Some(terrain) => {
            *terrain = Terrain::Level;
        }
    }

    visited.insert(pos);

    for direction in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let unit_vector = direction.unit_vector();
        if let Ok(next) = pos + unit_vector {
            if visited.contains(&next) {
                continue;
            }

            if grid.get_pos(next) == Some(&Terrain::Interior) {
                flood(grid, next, visited);
            }
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Terrain {
    Edge,
    Interior,
    Level,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Edge => write!(f, "#"),
            Terrain::Interior => write!(f, "#"),
            Terrain::Level => write!(f, "."),
        }
    }
}
