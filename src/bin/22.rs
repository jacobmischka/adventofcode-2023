use std::{collections::HashSet, io, str::FromStr};

fn main() {
    let mut cubes: Vec<Cube> = io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                return None;
            }

            Some(Cube::from_str(&line).unwrap())
        })
        .collect();

    cubes.sort();
    fall(&mut cubes);

    let mut part2 = 0;
    let part1 = (0..cubes.len()).fold(0, |acc, i| {
        if i == cubes.len() - 1
            || ((i + 1)..cubes.len()).all(|j| {
                !cubes[i].is_supporting(&cubes[j])
                    || (0..j).any(|k| k != i && cubes[k].is_supporting(&cubes[j]))
            })
        {
            acc + 1
        } else {
            let mut disintigrated = HashSet::new();
            disintigrated.insert(i);
            chain_reactors(&cubes, i, &mut disintigrated);
            part2 += disintigrated.len() - 1;
            acc
        }
    });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn chain_reactors(cubes: &[Cube], i: usize, disintigrated: &mut HashSet<usize>) {
    for j in (i + 1)..cubes.len() {
        if !disintigrated.contains(&j)
            && cubes[i].is_supporting(&cubes[j])
            && !(0..j).any(|k| !disintigrated.contains(&k) && cubes[k].is_supporting(&cubes[j]))
        {
            disintigrated.insert(j);
            if j < cubes.len() - 1 {
                chain_reactors(cubes, j, disintigrated);
            }
        }
    }
}

fn fall(cubes: &mut [Cube]) {
    cubes[0].1.z -= cubes[0].0.z;
    cubes[0].0.z = 0;

    for i in 1..cubes.len() {
        while !cubes[i].is_being_supported(&cubes[0..i]) && cubes[i].0.z > 0 && cubes[i].1.z > 0 {
            cubes[i].0.z -= 1;
            cubes[i].1.z -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube(Point, Point);

impl Cube {
    fn is_supporting(&self, other: &Cube) -> bool {
        (self.1.z + 1 == other.0.z) && {
            let min_self_x = self.0.x.min(self.1.x);
            let max_self_x = self.0.x.max(self.1.x);

            let min_self_y = self.0.y.min(self.1.y);
            let max_self_y = self.0.y.max(self.1.y);

            let min_other_x = other.0.x.min(other.1.x);
            let max_other_x = other.0.x.max(other.1.x);

            let min_other_y = other.0.y.min(other.1.y);
            let max_other_y = other.0.y.max(other.1.y);

            min_self_x <= max_other_x
                && max_self_x >= min_other_x
                && min_self_y <= max_other_y
                && max_self_y >= min_other_y
        }
    }

    fn is_being_supported(&self, others: &[Cube]) -> bool {
        others.iter().any(|other| other.is_supporting(self))
    }
}

impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split('~').map(Point::from_str);
        let cube = Cube(
            pieces
                .next()
                .ok_or_else(|| format!("not enough values in cube {s}"))??,
            pieces
                .next()
                .ok_or_else(|| format!("not enough values in cube {s}"))??,
        );

        if cube.0.z > cube.1.z {
            return Err(format!("second corner z coordinate lower than first {s}"));
        }

        Ok(cube)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z
            .cmp(&other.z)
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s
            .split(',')
            .map(|s| s.parse::<u32>().map_err(|_| format!("invalid point {s}")));
        Ok(Point {
            x: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
            y: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
            z: pieces
                .next()
                .ok_or_else(|| format!("not enough values in point {s}"))??,
        })
    }
}
