use std::{io, iter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            c => Err(format!("unknown spring {c}")),
        }
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut unknowns: Vec<usize> = Vec::new();
        let mut pieces = line.split_ascii_whitespace();
        let springs: Vec<Spring> = pieces
            .next()
            .unwrap()
            .char_indices()
            .map(|(i, c)| {
                let spring = Spring::try_from(c).unwrap();
                if spring == Spring::Unknown {
                    unknowns.push(i);
                }
                spring
            })
            .collect();
        let sizes: Vec<u32> = pieces
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        for n in 0..2usize.pow(unknowns.len() as u32) {
            let mut m = n;
            let mut potential_springs = springs.clone();
            for i in &unknowns {
                potential_springs[*i] = if m & 1 == 1 {
                    Spring::Damaged
                } else {
                    Spring::Operational
                };
                m = m >> 1;
            }

            if check_springs(&potential_springs, &sizes) {
                part1 += 1;
            }
        }

        let len = ((springs.len() + 1) * 5) - 1;
        let springs: Vec<_> = springs
            .into_iter()
            .chain(iter::once(Spring::Unknown))
            .cycle()
            .take(len)
            .collect();
        let len = sizes.len() * 5;
        let sizes: Vec<_> = sizes.into_iter().cycle().take(len).collect();
        let unknowns: Vec<_> = springs
            .iter()
            .enumerate()
            .filter_map(|(i, s)| if *s == Spring::Unknown { Some(i) } else { None })
            .collect();

        for n in 0..2u128.pow(unknowns.len() as u32) {
            let mut m = n;
            let mut potential_springs = springs.clone();
            for i in &unknowns {
                potential_springs[*i] = if m & 1 == 1 {
                    Spring::Damaged
                } else {
                    Spring::Operational
                };
                m = m >> 1;
            }

            if check_springs(&potential_springs, &sizes) {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn check_springs(springs: &Vec<Spring>, sizes: &Vec<u32>) -> bool {
    let mut size_i = 0;

    let mut damaged_size = 0;
    for spring in springs {
        match spring {
            Spring::Unknown => {
                return false;
            }
            Spring::Damaged => {
                damaged_size += 1;
            }
            Spring::Operational => {
                if damaged_size > 0 {
                    if let Some(current_size) = sizes.get(size_i) {
                        if *current_size != damaged_size {
                            return false;
                        }
                    } else {
                        return false;
                    }

                    size_i += 1;
                    damaged_size = 0;
                }
            }
        }
    }

    if damaged_size > 0 {
        if let Some(current_size) = sizes.get(size_i) {
            if *current_size != damaged_size {
                return false;
            }
        } else {
            return false;
        }

        size_i += 1;
    }

    size_i == sizes.len()
}
