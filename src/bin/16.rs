use std::{collections::HashSet, io};

use adventofcode_2023::grid::{Actor, Direction, Grid, Position};

fn main() {
    let grid = Grid(
        io::stdin()
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();

                if line.is_empty() {
                    return None;
                } else {
                    Some(line.chars().collect())
                }
            })
            .collect(),
    );

    let mut energized = energize_grid(
        &grid,
        Actor {
            pos: Position(0, 0),
            vector: Direction::East.unit_vector(),
        },
    );
    println!("Part 1: {}", energized);

    for x in 0..grid[0].len() {
        if x > 0 {
            energized = energize_grid(
                &grid,
                Actor {
                    pos: Position(x, 0),
                    vector: Direction::South.unit_vector(),
                },
            )
            .max(energized);
        }
        energized = energize_grid(
            &grid,
            Actor {
                pos: Position(x, grid.len() - 1),
                vector: Direction::North.unit_vector(),
            },
        )
        .max(energized);
    }

    for y in 0..grid.len() {
        energized = energize_grid(
            &grid,
            Actor {
                pos: Position(0, y),
                vector: Direction::East.unit_vector(),
            },
        )
        .max(energized);
        energized = energize_grid(
            &grid,
            Actor {
                pos: Position(grid[y].len() - 1, y),
                vector: Direction::West.unit_vector(),
            },
        )
        .max(energized);
    }

    println!("Part 2: {}", energized);
}

fn energize_grid(grid: &Grid<char>, beam: Actor) -> usize {
    let mut seen_beams: HashSet<Actor> = HashSet::new();
    let mut energized: HashSet<Position> = HashSet::new();
    let mut beams: Vec<Actor> = vec![beam];
    let mut new_beams: Vec<Actor> = Vec::new();

    while !beams.is_empty() {
        beams.retain_mut(|beam| {
            seen_beams.insert(beam.clone());

            if let Some(tile) = grid.get_pos(beam.pos) {
                energized.insert(beam.pos);
                match tile {
                    '.' | '#' => {}
                    '/' => match beam.vector.direction().unwrap() {
                        Direction::North => {
                            beam.vector = Direction::East.unit_vector();
                        }
                        Direction::South => {
                            beam.vector = Direction::West.unit_vector();
                        }
                        Direction::East => {
                            beam.vector = Direction::North.unit_vector();
                        }
                        Direction::West => {
                            beam.vector = Direction::South.unit_vector();
                        }
                    },
                    '\\' => match beam.vector.direction().unwrap() {
                        Direction::North => {
                            beam.vector = Direction::West.unit_vector();
                        }
                        Direction::South => {
                            beam.vector = Direction::East.unit_vector();
                        }
                        Direction::East => {
                            beam.vector = Direction::South.unit_vector();
                        }
                        Direction::West => {
                            beam.vector = Direction::North.unit_vector();
                        }
                    },
                    '|' => match beam.vector.direction().unwrap() {
                        Direction::West | Direction::East => {
                            beam.vector = Direction::North.unit_vector();
                            new_beams.push(Actor {
                                pos: beam.pos,
                                vector: Direction::South.unit_vector(),
                            });
                        }
                        _ => {}
                    },
                    '-' => match beam.vector.direction().unwrap() {
                        Direction::North | Direction::South => {
                            beam.vector = Direction::East.unit_vector();
                            new_beams.push(Actor {
                                pos: beam.pos,
                                vector: Direction::West.unit_vector(),
                            });
                        }
                        _ => {}
                    },
                    c => panic!("invalid tile {c}"),
                }

                if beam.do_move().is_err() || seen_beams.contains(&beam) {
                    return false;
                }

                true
            } else {
                false
            }
        });

        if !new_beams.is_empty() {
            beams.append(&mut new_beams);
        }
    }

    energized.len()
}
