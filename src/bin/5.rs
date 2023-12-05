use std::{io, ops};

#[derive(Clone, Debug)]
struct ValueRange(ops::Range<u64>);

impl ValueRange {
    fn intersection(&self, other: &ValueRange) -> (Option<ValueRange>, Vec<ValueRange>) {
        if self.0.start < other.0.end && self.0.end > other.0.start {
            let intersection =
                ValueRange(self.0.start.max(other.0.start)..self.0.end.min(other.0.end));
            let mut rem = Vec::new();

            if self.0.start < other.0.start {
                rem.push(ValueRange(self.0.start..other.0.start));
            }

            if self.0.end > other.0.end {
                rem.push(ValueRange(other.0.end..self.0.end));
            }

            return (Some(intersection), rem);
        }

        (None, vec![self.clone()])
    }
}

fn main() {
    let mut working_vals: Vec<u64> = Vec::new();
    let mut next_vals: Vec<u64> = Vec::new();

    let mut working_ranges: Vec<ValueRange> = Vec::new();
    let mut next_ranges: Vec<ValueRange> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() || line.ends_with("map:") {
            if !next_vals.is_empty() {
                working_vals.append(&mut next_vals);
            }
            if !next_ranges.is_empty() {
                working_ranges.append(&mut next_ranges);
            }
            continue;
        }

        if line.starts_with("seeds:") {
            working_vals = line
                .split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let mut i = 0;
            while i < working_vals.len() {
                working_ranges.push(ValueRange(
                    working_vals[i]..(working_vals[i] + working_vals[i + 1]),
                ));
                i += 2;
            }

            continue;
        }

        if working_vals.is_empty() {
            continue;
        }

        let mut nums = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap());
        let dest_start = nums.next().unwrap();
        let source_start = nums.next().unwrap();
        let range_len = nums.next().unwrap();
        working_vals.retain(|&val| {
            if source_start <= val && val < source_start + range_len {
                next_vals.push(dest_start + (val - source_start));
                println!("{val} -> {}", dest_start + (val - source_start));
                return false;
            }

            true
        });

        // not good but good enough
        working_ranges = working_ranges
            .into_iter()
            .flat_map(|working_range| {
                let (intersection, rem) = working_range
                    .intersection(&ValueRange(source_start..(source_start + range_len)));

                if let Some(intersection) = intersection {
                    if dest_start > source_start {
                        let diff = dest_start - source_start;
                        next_ranges.push(ValueRange(
                            (intersection.0.start + diff)..(intersection.0.end + diff),
                        ))
                    } else {
                        let diff = source_start - dest_start;
                        next_ranges.push(ValueRange(
                            (intersection.0.start - diff)..(intersection.0.end - diff),
                        ))
                    }
                }

                return rem;
            })
            .collect();
    }

    if !next_vals.is_empty() {
        working_vals.append(&mut next_vals);
    }
    if !next_ranges.is_empty() {
        working_ranges.append(&mut next_ranges);
    }

    let part1 = working_vals.iter().reduce(|acc, x| acc.min(x)).unwrap();
    let part2 = working_ranges
        .iter()
        .reduce(|acc, x| if x.0.start < acc.0.start { x } else { acc })
        .unwrap()
        .0
        .start;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
