use std::{collections::HashMap, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(format!("invalid direction: {c}")),
        }
    }
}

fn main() {
    let mut lines = io::stdin().lines();
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| Direction::try_from(c).unwrap())
        .collect();

    let map: HashMap<String, (String, String)> = lines
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let mut halves = line.split(" = ");
            let key = halves.next().unwrap().to_string();

            let stripped = halves.next().unwrap().replace(&['(', ')', ','], "");
            let mut dirs = stripped.split_ascii_whitespace().map(String::from);

            (key, (dirs.next().unwrap(), dirs.next().unwrap()))
        })
        .collect();

    let mut pos = "AAA";
    let mut dirs = directions.iter().cycle();

    let mut part1 = 0;
    while pos != "ZZZ" {
        let options = map.get(pos).unwrap();
        let dir = *dirs.next().unwrap();
        pos = if dir == Direction::Left {
            &options.0
        } else {
            &options.1
        };
        part1 += 1;
    }

    println!("Part 1: {part1}");

    let mut ghost_positions: Vec<&str> = map
        .keys()
        .filter(|pos| pos.ends_with('A'))
        .map(|s| s.as_str())
        .collect();
    let mut dirs = directions.iter().cycle();

    let mut step: u64 = 0;
    let mut seen: Vec<HashMap<&str, u64>> = vec![HashMap::new(); ghost_positions.len()];
    let mut cycles: Vec<Option<u64>> = vec![None; ghost_positions.len()];
    while !ghost_positions.iter().all(|pos| pos.ends_with('Z'))
        && !cycles.iter().all(|c| c.is_some())
    {
        let dir = *dirs.next().unwrap();
        for i in 0..ghost_positions.len() {
            if cycles[i].is_some() {
                continue;
            }

            let options = map.get(ghost_positions[i]).unwrap();
            ghost_positions[i] = if dir == Direction::Left {
                &options.0
            } else {
                &options.1
            };

            if ghost_positions[i].ends_with('Z') {
                if seen[i].contains_key(ghost_positions[i]) {
                    if cycles[i].is_none() {
                        let first_seen = *seen[i].get(ghost_positions[i]).unwrap();
                        cycles[i] = Some(step - first_seen);
                    }
                } else {
                    seen[i].insert(ghost_positions[i], step);
                }
            }
        }
        step += 1;
    }

    let part2 = cycles
        .into_iter()
        .map(|o| o.unwrap())
        .fold(1, |acc, x| lcm(acc, x));

    println!("Part 2: {}", part2);
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else if b > a {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}
