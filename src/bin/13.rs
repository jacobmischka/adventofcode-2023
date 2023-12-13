use std::{collections::HashSet, io};

use adventofcode_2023::grid::{Grid, Position};

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut pattern = Grid(Vec::new());
    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if pattern.is_empty() {
                continue;
            }

            let (reflection, runner_up) = get_reflection(&pattern).unwrap();
            if runner_up.1.len() != 2 {
                panic!("invalid smudge reflection {:?}", runner_up);
            }

            match reflection.kind {
                ReflectionKind::Horizontal => {
                    part1 += reflection.index * 100;
                }
                ReflectionKind::Vertical => {
                    part1 += reflection.index;
                }
            }

            match runner_up.0.kind {
                ReflectionKind::Horizontal => {
                    part2 += runner_up.0.index * 100;
                }
                ReflectionKind::Vertical => {
                    part2 += runner_up.0.index;
                }
            }

            pattern.clear();
        } else {
            pattern.push(line.chars().collect());
        }
    }

    if !pattern.is_empty() {
        let (reflection, runner_up) = get_reflection(&pattern).unwrap();
        if runner_up.1.len() != 2 {
            panic!("invalid smudge reflection {:?}", runner_up);
        }

        match reflection.kind {
            ReflectionKind::Horizontal => {
                part1 += reflection.index * 100;
            }
            ReflectionKind::Vertical => {
                part1 += reflection.index;
            }
        }

        match runner_up.0.kind {
            ReflectionKind::Horizontal => {
                part2 += runner_up.0.index * 100;
            }
            ReflectionKind::Vertical => {
                part2 += runner_up.0.index;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReflectionKind {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct Reflection {
    kind: ReflectionKind,
    index: usize,
}

fn get_reflection(
    pattern: &Grid,
) -> Result<(Reflection, (Reflection, HashSet<Position>)), (Reflection, HashSet<Position>)> {
    let mut reflection: Option<Reflection> = None;
    let mut runner_up: Option<(Reflection, HashSet<Position>)> = None;
    // check rows
    let len = pattern.len();
    for y in 1..len {
        let mut differences: HashSet<Position> = HashSet::new();
        let reflection_len = y.min(len - y);
        for i in 0..=reflection_len {
            for x in 0..pattern[0].len() {
                if pattern[y - i][x] != pattern[y + i - 1][x] {
                    differences.insert(Position(x, y - i));
                    differences.insert(Position(x, y + i - 1));
                }
            }
        }

        let potential_reflection = Reflection {
            kind: ReflectionKind::Horizontal,
            index: y,
        };

        if differences.is_empty() {
            reflection = Some(potential_reflection);
        } else {
            match &runner_up {
                None => {
                    runner_up = Some((potential_reflection, differences));
                }
                Some((_, min_differences)) => {
                    if differences.len() < min_differences.len() {
                        runner_up = Some((potential_reflection, differences));
                    }
                }
            }
        }
    }

    // check columns
    let len = pattern[0].len();
    for x in 1..len {
        let mut differences: HashSet<Position> = HashSet::new();
        let reflection_len = x.min(len - x);
        for i in 0..=reflection_len {
            for (y, row) in pattern.iter().enumerate() {
                if row[x - i] != row[x + i - 1] {
                    differences.insert(Position(x - i, y));
                    differences.insert(Position(x + i - 1, y));
                }
            }
        }

        let potential_reflection = Reflection {
            kind: ReflectionKind::Vertical,
            index: x,
        };

        if differences.is_empty() {
            reflection = Some(potential_reflection);
        } else {
            match &runner_up {
                None => {
                    runner_up = Some((potential_reflection, differences));
                }
                Some((_, min_differences)) => {
                    if differences.len() < min_differences.len() {
                        runner_up = Some((potential_reflection, differences));
                    }
                }
            }
        }
    }

    match reflection {
        Some(reflection) => Ok((reflection, runner_up.unwrap())),
        None => Err(runner_up.unwrap()),
    }
}
