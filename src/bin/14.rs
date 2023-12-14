use std::{collections::HashMap, io};

use adventofcode_2023::grid::{Direction, Grid};

const TOTAL_CYCLES: usize = 1000000000;

fn main() {
    let mut grid = Grid(
        io::stdin()
            .lines()
            .filter_map(|line| {
                let row: Vec<_> = line.unwrap().chars().collect();
                if row.is_empty() {
                    None
                } else {
                    Some(row)
                }
            })
            .collect(),
    );

    tilt(&mut grid, Direction::North);

    println!("Part 1: {}", total_load(&grid));

    tilt(&mut grid, Direction::West);
    tilt(&mut grid, Direction::South);
    tilt(&mut grid, Direction::East);

    let mut seen: HashMap<Grid, usize> = HashMap::new();

    let mut cycles = 1;
    while !seen.contains_key(&grid) && cycles < TOTAL_CYCLES {
        seen.insert(grid.clone(), cycles);
        cycles += 1;
        cycle(&mut grid);
    }
    if cycles < TOTAL_CYCLES {
        let cycle_size = cycles - seen.get(&grid).unwrap();
        let rem = (TOTAL_CYCLES - cycles) % cycle_size;
        for _ in 0..rem {
            cycle(&mut grid);
        }
    }

    println!("Part 2: {}", total_load(&grid));
}

fn total_load(grid: &Grid) -> usize {
    grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + ((grid.len() - i)
            * row
                .iter()
                .fold(0, |acc, x| if *x == 'O' { acc + 1 } else { acc }))
    })
}

fn cycle(grid: &mut Grid) {
    tilt(grid, Direction::North);
    tilt(grid, Direction::West);
    tilt(grid, Direction::South);
    tilt(grid, Direction::East);
}

fn tilt(grid: &mut Grid, direction: Direction) {
    match direction {
        Direction::North => {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == 'O' {
                        let mut new_y = y;
                        while new_y > 0 && grid[new_y - 1][x] == '.' {
                            grid[new_y - 1][x] = grid[new_y][x];
                            grid[new_y][x] = '.';
                            new_y -= 1;
                        }
                    }
                }
            }
        }
        Direction::South => {
            for y in (0..grid.len()).rev() {
                for x in (0..grid[y].len()).rev() {
                    if grid[y][x] == 'O' {
                        let mut new_y = y;
                        while new_y < grid.len() - 1 && grid[new_y + 1][x] == '.' {
                            grid[new_y + 1][x] = grid[new_y][x];
                            grid[new_y][x] = '.';
                            new_y += 1;
                        }
                    }
                }
            }
        }
        Direction::East => {
            for y in (0..grid.len()).rev() {
                for x in (0..grid[y].len()).rev() {
                    if grid[y][x] == 'O' {
                        let mut new_x = x;
                        while new_x < grid[y].len() - 1 && grid[y][new_x + 1] == '.' {
                            grid[y][new_x + 1] = grid[y][new_x];
                            grid[y][new_x] = '.';
                            new_x += 1;
                        }
                    }
                }
            }
        }
        Direction::West => {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == 'O' {
                        let mut new_x = x;
                        while new_x > 0 && grid[y][new_x - 1] == '.' {
                            grid[y][new_x - 1] = grid[y][new_x];
                            grid[y][new_x] = '.';
                            new_x -= 1;
                        }
                    }
                }
            }
        }
    }
}
