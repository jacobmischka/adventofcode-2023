use std::{
    collections::{HashMap, HashSet},
    io,
};

use adventofcode_2023::grid::*;

fn main() {
    let mut start: Option<Position> = None;
    let grid = Grid(
        io::stdin()
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                let line = line.unwrap();
                if line.is_empty() {
                    None
                } else {
                    Some(
                        line.chars()
                            .enumerate()
                            .map(|(j, c)| {
                                if c == 'S' {
                                    start = Some(Position(j, i));
                                }

                                c
                            })
                            .collect(),
                    )
                }
            })
            .collect(),
    );

    let start = start.unwrap();
    let mut pipe_loop: HashMap<Position, usize> = HashMap::new();
    pipe_loop.insert(start, 0);

    let mut prev = start;
    let mut pos = {
        let mut pos: Option<Position> = None;
        'outer: for dy in (-1)..=1 {
            for dx in (-1)..=1 {
                if let Ok(potential_pos) = start + Vector(dx, dy) {
                    if let Ok(vectors) = tile_ends(grid.get_pos(potential_pos).unwrap()) {
                        if vectors
                            .iter()
                            .any(|vector| potential_pos + *vector == Ok(start))
                        {
                            pos = Some(potential_pos);
                            break 'outer;
                        }
                    }
                }
            }
        }

        pos.unwrap()
    };

    let mut step = 1;
    pipe_loop.insert(pos, step);
    while pos != start {
        step += 1;

        let next_pos = tile_ends(grid[pos.1][pos.0])
            .unwrap()
            .into_iter()
            .filter_map(|vector| (pos + vector).ok())
            .find(|next_pos| *next_pos != prev)
            .unwrap();
        prev = pos;
        pos = next_pos;

        pipe_loop.insert(pos, step);
    }

    println!("Part 1: {}", step / 2 + step % 2);

    let mut outside_visited: HashSet<Position> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if y != 0 && y != grid.len() - 1 && x != 0 && x != grid[y].len() - 1 {
                continue;
            }

            let pos = Position(x, y);
            if !pipe_loop.contains_key(&pos) {
                outside_visited.insert(pos);
                flood(pos, &grid, &pipe_loop, &mut outside_visited);
            }
        }
    }

    let mut part2 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let pos = Position(x, y);
            if !pipe_loop.contains_key(&pos) && !outside_visited.contains(&pos) {
                println!("contained: {pos:?}");
                part2 += 1;
            }
        }
    }

    println!("Part 2: {part2}");
}

// fn can_visit(
//     from: Position,
//     vector: Vector,
//     grid: &Grid,
//     pipe_loop: &HashMap<Position, usize>,
// ) -> bool {
//     match from + vector {
//         Some(to) => {
//
//         },
//         None => false,
//     }
// }

fn flood(
    pos: Position,
    grid: &Grid,
    pipe_loop: &HashMap<Position, usize>,
    visited: &mut HashSet<Position>,
) {
    for y in -1..=1 {
        for x in -1..=1 {
            let vector = Vector(x as isize, y as isize);
            if vector.manhattan_distance() > 1 {
                continue;
            }

            if let Ok(potential_pos) = pos + vector {
                if !visited.contains(&potential_pos) {
                    if let Some(potential_tile) = grid.get_pos(potential_pos) {
                        if !pipe_loop.contains_key(&potential_pos) || {
                            let potential_step = *pipe_loop.get(&potential_pos).unwrap();

                            if vector.0 == 0 {
                                (match potential_pos + Vector(-1, 0) {
                                    Ok(adj) => match pipe_loop.get(&adj) {
                                        Some(adj_step) => adj_step.abs_diff(potential_step) != 1,
                                        _ => false,
                                    },
                                    _ => false,
                                }) || (match potential_pos + Vector(1, 0) {
                                    Ok(adj) => match pipe_loop.get(&adj) {
                                        Some(adj_step) => adj_step.abs_diff(potential_step) != 1,
                                        _ => false,
                                    },
                                    _ => false,
                                })
                            } else {
                                (match potential_pos + Vector(0, -1) {
                                    Ok(adj) => match pipe_loop.get(&adj) {
                                        Some(adj_step) => adj_step.abs_diff(potential_step) != 1,
                                        _ => false,
                                    },
                                    _ => false,
                                }) || (match potential_pos + Vector(0, 1) {
                                    Ok(adj) => match pipe_loop.get(&adj) {
                                        Some(adj_step) => adj_step.abs_diff(potential_step) != 1,
                                        _ => false,
                                    },
                                    _ => false,
                                })
                            }
                        } {
                            visited.insert(potential_pos);
                            println!("visited {potential_pos:?}");
                            flood(potential_pos, grid, pipe_loop, visited);
                        }
                    }
                }
            }
        }
    }
}

fn tile_ends(tile: char) -> Result<[Vector; 2], String> {
    match tile {
        '|' => Ok([Vector(0, -1), Vector(0, 1)]),
        '-' => Ok([Vector(-1, 0), Vector(1, 0)]),
        'L' => Ok([Vector(0, -1), Vector(1, 0)]),
        'J' => Ok([Vector(0, -1), Vector(-1, 0)]),
        '7' => Ok([Vector(-1, 0), Vector(0, 1)]),
        'F' => Ok([Vector(0, 1), Vector(1, 0)]),
        _ => Err(format!("invalid tile: {tile}")),
    }
}
