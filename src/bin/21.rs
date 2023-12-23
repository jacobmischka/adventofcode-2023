use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io,
};

use adventofcode_2023::grid::{Direction, Grid, Position, SignedPosition};

fn main() {
    let mut start: Option<Position> = None;
    let grid = Grid::new(
        io::stdin()
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                let line = line.unwrap();

                if line.is_empty() {
                    return None;
                }

                Some(
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| match c {
                            'S' => {
                                start = Some(Position(x, y));
                                Tile::GardenPlot
                            }
                            '.' => Tile::GardenPlot,
                            '#' => Tile::Rock,
                            c => panic!("invalid tile {c}"),
                        })
                        .collect(),
                )
            })
            .collect(),
    );

    let start = start.unwrap();

    let mut states: HashSet<(Position, u32)> = HashSet::new();
    step(&grid, start, 64, &mut states);

    println!(
        "Part 1: {}",
        states
            .iter()
            .fold(0, |acc, x| if x.1 == 0 { acc + 1 } else { acc })
    );

    let start = start.to_signed();
    let mut states: HashMap<(SignedPosition, u32), Vec<SignedPosition>> = HashMap::new();
    // let total_steps = 26501365;
    let total_steps = 50;
    infinite_step(&grid, start, total_steps, &mut states);

    println!(
        "Part 2: {}",
        states.get(&(start, total_steps)).unwrap().len()
    );
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::GardenPlot => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

fn step(
    grid: &Grid<Tile>,
    pos: Position,
    steps_remaining: u32,
    states: &mut HashSet<(Position, u32)>,
) {
    if states.contains(&(pos, steps_remaining)) {
        return;
    }

    states.insert((pos, steps_remaining));

    if steps_remaining == 0 {
        return;
    }

    for dir in Direction::all() {
        if let Ok(new_pos) = pos + dir.unit_vector() {
            match grid.get_pos(new_pos) {
                None => {}
                Some(Tile::Rock) => {}
                Some(Tile::GardenPlot) => {
                    step(grid, new_pos, steps_remaining - 1, states);
                }
            }
        }
    }
}

fn infinite_step(
    grid: &Grid<Tile>,
    pos: SignedPosition,
    steps_remaining: u32,
    states: &mut HashMap<(SignedPosition, u32), Vec<SignedPosition>>,
) -> Vec<SignedPosition> {
    let wrapped_pos = grid.wrapped_position(pos);
    let signed_wrapped_pos = wrapped_pos.to_signed();

    if let Some(end_positions) = states.get(&(signed_wrapped_pos, steps_remaining)) {
        let offset = pos - signed_wrapped_pos;
        return end_positions.iter().map(|p| *p + offset).collect();
    }

    let result: Vec<_> = Direction::all()
        .into_iter()
        .flat_map(|dir| {
            let unit_vector = dir.unit_vector();
            let new_pos = pos + unit_vector;
            let new_wrapped_pos = grid.wrapped_position(new_pos);

            match grid.get_pos(new_wrapped_pos) {
                None => {
                    panic!("wrapping failed, position {:?}", new_pos);
                }
                Some(Tile::Rock) => Vec::new(),
                Some(Tile::GardenPlot) => infinite_step(grid, new_pos, steps_remaining - 1, states),
            }
        })
        .collect();

    states.insert((signed_wrapped_pos, steps_remaining), result.clone());

    result
}
