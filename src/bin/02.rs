use std::{collections::HashMap, io};

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut limits: HashMap<&str, u32> = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    for game in io::stdin().lines() {
        let line = game.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut pieces = line.split(": ");
        let prelude = pieces.next().unwrap();
        let id: u32 = prelude
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let mut maxes: HashMap<&str, u32> = HashMap::new();
        for round in pieces.next().unwrap().trim().split("; ") {
            for cube in round.split(", ") {
                let mut words = cube.split_ascii_whitespace();
                let count: u32 = words.next().unwrap().parse().unwrap();
                let color = words.next().unwrap();

                let max = maxes.entry(color).or_default();
                *max = (*max).max(count);
            }
        }

        if maxes.iter().all(|(color, count)| match limits.get(color) {
            Some(max) => *count <= *max,
            None => false,
        }) {
            part1 += id;
        }

        part2 += maxes.into_values().fold(1, |acc, val| acc * val);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
