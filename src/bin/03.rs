use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io,
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point(i32, i32);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct SchematicNumber {
    value: u32,
    start: Point,
    end: Point,
}

fn main() {
    let mut nums: Vec<SchematicNumber> = Vec::new();
    let mut symbols: HashMap<Point, char> = HashMap::new();

    for (y, line) in io::stdin().lines().enumerate() {
        let mut buf = String::new();
        let mut start: Option<Point> = None;
        let mut max_x = 0;
        for (x, c) in line.unwrap().chars().enumerate() {
            max_x = x;
            if c.is_ascii_digit() {
                if buf.is_empty() {
                    start = Some(Point(x as _, y as _));
                }
                buf.push(c);
            } else {
                match start {
                    Some(s) => {
                        nums.push(SchematicNumber {
                            value: buf.parse().unwrap(),
                            start: s,
                            end: Point(x as i32 - 1, y as _),
                        });
                        buf.clear();
                        start = None;
                    }
                    None => {}
                }
                if c != '.' {
                    symbols.insert(Point(x as _, y as _), c);
                }
            }
        }

        match start {
            Some(s) => {
                nums.push(SchematicNumber {
                    value: buf.parse().unwrap(),
                    start: s,
                    end: Point(max_x as i32 - 1, y as _),
                });
            }
            None => {}
        }
    }

    let mut num_pos: HashMap<Point, &SchematicNumber> = HashMap::new();

    let mut part1 = 0;
    for num in &nums {
        let mut added = false;
        for y in (num.start.1 - 1)..=(num.end.1 + 1) {
            for x in (num.start.0 - 1)..=(num.end.0 + 1) {
                let point = Point(x, y);

                if !added && symbols.contains_key(&point) {
                    part1 += num.value;
                    added = true;
                }

                if y == num.start.1 && num.start.0 <= x && x <= num.end.0 {
                    num_pos.insert(point, num);
                }
            }
        }
    }

    let mut part2 = 0;
    for (point, symbol) in symbols {
        if symbol == '*' {
            let mut adj: HashSet<&SchematicNumber> = HashSet::new();
            for y in (point.1 - 1)..=(point.1 + 1) {
                for x in (point.0 - 1)..=(point.0 + 1) {
                    if let Some(num) = num_pos.get(&Point(x, y)) {
                        adj.insert(num);
                    }
                }
            }

            if adj.len() == 2 {
                part2 += adj.into_iter().fold(1, |acc, x| acc * x.value);
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
