use std::{collections::HashSet, io};

use adventofcode_2023::grid::*;

fn main() {
    let mut cols_with_galaxies: HashSet<usize> = HashSet::new();
    let mut rows_with_galaxies: HashSet<usize> = HashSet::new();

    let mut galaxies: Vec<Position> = Vec::new();
    let image = Grid(
        io::stdin()
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                let line = line.unwrap();
                if line.is_empty() {
                    None
                } else {
                    Some(
                        line.chars()
                            .enumerate()
                            .map(|(x, c)| {
                                if c == '#' {
                                    cols_with_galaxies.insert(x);
                                    rows_with_galaxies.insert(y);
                                    galaxies.push(Position(x, y));
                                }

                                c
                            })
                            .collect(),
                    )
                }
            })
            .collect(),
    );

    let cols_to_expand: HashSet<usize> = (0..image.len())
        .filter(|i| !cols_with_galaxies.contains(i))
        .collect();
    let rows_to_expand: HashSet<usize> = (0..image[0].len())
        .filter(|i| !rows_with_galaxies.contains(i))
        .collect();

    let mut part1 = 0;
    let mut part2: u128 = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            for x in galaxies[i].0.min(galaxies[j].0)..galaxies[i].0.max(galaxies[j].0) {
                if cols_to_expand.contains(&x) {
                    part1 += 2;
                    part2 += 1_000_000;
                } else {
                    part1 += 1;
                    part2 += 1;
                }
            }

            for y in galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1) {
                if rows_to_expand.contains(&y) {
                    part1 += 2;
                    part2 += 1_000_000;
                } else {
                    part1 += 1;
                    part2 += 1;
                }
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
