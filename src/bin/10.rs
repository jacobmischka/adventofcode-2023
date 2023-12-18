use std::{
    collections::{HashMap, HashSet},
    io,
};

use adventofcode_2023::grid::*;

fn main() {
    let mut start: Option<Position> = None;
    let mut start_vectors: Vec<Vector> = Vec::new();
    let mut grid = Grid(
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
                    if let Ok(vectors) = tile_ends(*grid.get_pos(potential_pos).unwrap()) {
                        for vector in vectors {
                            if potential_pos + vector == Ok(start) {
                                pos = Some(potential_pos);
                                start_vectors.push(Vector(vector.0 * -1, vector.1 * -1));
                                break 'outer;
                            }
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
    start_vectors.push(prev - pos);

    println!("Part 1: {}", step / 2 + step % 2);

    for possible_tile in ['|', '-', 'L', 'J', '7', 'F'] {
        let vectors = tile_ends(possible_tile).unwrap();
        if vectors.into_iter().all(|v| start_vectors.contains(&v)) {
            grid[start.1][start.0] = possible_tile;
            break;
        }
    }

    let mut inside_positions: HashSet<Position> = HashSet::new();
    for y in 0..grid.len() {
        let mut inside = false;
        for x in 0..grid[y].len() {
            let pos = Position(x, y);
            if pipe_loop.contains_key(&pos) {
                let tile = *grid.get_pos(pos).unwrap();
                let vs = tile_ends(tile).unwrap();
                if vs.into_iter().any(|v| v.1 < 0) {
                    inside = !inside;
                }
            } else {
                if inside {
                    inside_positions.insert(pos);
                }
            }
        }
    }

    println!("Part 2: {}", inside_positions.len());
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
