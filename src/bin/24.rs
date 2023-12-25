use std::{io, str::FromStr};

use adventofcode_2023::{Point, Point3D, Vector3D};

fn main() {
    let hailstones: Vec<Hailstone> = io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(Hailstone::from_str(&line).unwrap())
            }
        })
        .collect();

    let bb_min = 200000000000000.0;
    let bb_max = 400000000000000.0;

    let mut part1 = 0;
    for i in 0..(hailstones.len() - 1) {
        let h1 = &hailstones[i];
        for j in (i + 1)..hailstones.len() {
            let h2 = &hailstones[j];
            if h1.vel.y * h2.vel.x == h2.vel.y * h1.vel.x {
                continue;
            }

            let t1 = (h2.vel.y * (h1.pos.x - h2.pos.x) - h2.vel.x * (h1.pos.y - h2.pos.y))
                / (h1.vel.y * h2.vel.x - h1.vel.x * h2.vel.y);
            let t2 = (h1.vel.y * (h2.pos.x - h1.pos.x) - h1.vel.x * (h2.pos.y - h1.pos.y))
                / (h2.vel.y * h1.vel.x - h2.vel.x * h1.vel.y);

            let intersection = Point(h1.pos.x + t1 * h1.vel.x, h1.pos.y + t1 * h1.vel.y);

            if t1 > 0.0
                && bb_min < intersection.0
                && intersection.0 < bb_max
                && t2 > 0.0
                && bb_min < intersection.1
                && intersection.1 < bb_max
            {
                part1 += 1;
            }
        }
    }

    println!("Part 1: {part1}");
}

#[derive(Debug, Clone)]
struct Hailstone {
    pos: Point3D<f64>,
    vel: Vector3D<f64>,
}

impl FromStr for Hailstone {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(" @ ");
        let pos = Point3D::from_str(
            halves
                .next()
                .ok_or_else(|| format!("invalid hailstone {s}"))?,
        )?;
        let vel = Vector3D::from_str(
            halves
                .next()
                .ok_or_else(|| format!("invalid hailstone {s}"))?,
        )?;
        Ok(Hailstone { pos, vel })
    }
}
