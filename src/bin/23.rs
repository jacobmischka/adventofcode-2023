use std::{collections::HashSet, fmt::Display, io};

use adventofcode_2023::grid::{Direction, Grid, Position};

fn main() {
    let grid = Grid::new(
        io::stdin()
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();
                if line.is_empty() {
                    None
                } else {
                    Some(line.chars().map(|c| Tile::try_from(c).unwrap()).collect())
                }
            })
            .collect(),
    );

    let start = Position(1, 0);
    let mut trodden: HashSet<Position> = HashSet::new();
    walk(&grid, start, &mut trodden, true);
    println!("Part 1: {}", trodden.len() - 1);

    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
    //         let pos = Position(x, y);
    //         if trodden.contains(&pos) {
    //             print!("O");
    //         } else {
    //             print!("{}", grid.get_pos(pos).unwrap());
    //         }
    //     }
    //     println!();
    // }

    trodden.clear();
    walk(&grid, start, &mut trodden, false);
    println!("Part 2: {}", trodden.len() - 1);
}

fn walk(grid: &Grid<Tile>, mut pos: Position, trodden: &mut HashSet<Position>, is_slippery: bool) {
    trodden.insert(pos);
    let mut options = possible_steps(grid, pos, trodden, is_slippery);

    while options.len() == 1 {
        pos = options[0];
        trodden.insert(pos);
        options = possible_steps(grid, pos, trodden, is_slippery);
    }

    let mut best_trodden: Option<HashSet<Position>> = None;
    for option in options {
        let mut new_trodden = trodden.clone();
        walk(grid, option, &mut new_trodden, is_slippery);
        match &mut best_trodden {
            None => {
                best_trodden = Some(new_trodden);
            }
            Some(best) => {
                if new_trodden.len() > best.len() {
                    *best = new_trodden;
                }
            }
        }
    }

    if let Some(best) = best_trodden {
        *trodden = best;
    }
}

fn possible_steps(
    grid: &Grid<Tile>,
    pos: Position,
    trodden: &HashSet<Position>,
    is_slippery: bool,
) -> Vec<Position> {
    if is_slippery {
        match grid.get_pos(pos) {
            Some(Tile::Slope(dir)) => {
                return (pos + dir.unit_vector())
                    .map(|new_pos| {
                        if trodden.contains(&new_pos) {
                            Vec::new()
                        } else {
                            vec![new_pos]
                        }
                    })
                    .unwrap_or_else(|_| Vec::new());
            }
            _ => {}
        }
    }

    Direction::all()
        .into_iter()
        .filter_map(|dir| {
            (pos + dir.unit_vector()).ok().and_then(|new_pos| {
                match (trodden.contains(&new_pos), grid.get_pos(new_pos)) {
                    (false, Some(tile)) => {
                        if *tile != Tile::Forest {
                            return Some(new_pos);
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Path),
            '#' => Ok(Tile::Forest),
            '^' => Ok(Tile::Slope(Direction::North)),
            '>' => Ok(Tile::Slope(Direction::East)),
            'v' => Ok(Tile::Slope(Direction::South)),
            '<' => Ok(Tile::Slope(Direction::West)),
            c => Err(format!("invalid tile {c}")),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Path => write!(f, "."),
            Tile::Forest => write!(f, "#"),
            Tile::Slope(Direction::North) => write!(f, "^"),
            Tile::Slope(Direction::East) => write!(f, ">"),
            Tile::Slope(Direction::South) => write!(f, "v"),
            Tile::Slope(Direction::West) => write!(f, "<"),
        }
    }
}
