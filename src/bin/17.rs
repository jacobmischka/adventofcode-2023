use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io,
};

use adventofcode_2023::grid::{Direction, Grid, Position, RelativeDirection};

fn main() {
    let grid: Grid<u32> = Grid(
        io::stdin()
            .lines()
            .filter_map(|line| {
                let line = line.unwrap();
                if line.is_empty() {
                    None
                } else {
                    Some(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                }
            })
            .collect(),
    );

    let part1 = crucible_dijkstra(
        &grid,
        Position(0, 0),
        Position(grid.last().unwrap().len() - 1, grid.len() - 1),
        CrucibleType::Basic,
    )
    .unwrap();

    println!("Part 1: {}", part1.dist);

    let part2 = crucible_dijkstra(
        &grid,
        Position(0, 0),
        Position(grid.last().unwrap().len() - 1, grid.len() - 1),
        CrucibleType::Ultra,
    )
    .unwrap();

    println!("Part 2: {}", part2.dist);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathState {
    dist: u64,
    pos: Position,
    last_move: (Direction, u8),
    path: Vec<(Position, Direction)>,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.last_move.cmp(&other.last_move))
            .then_with(|| self.path.cmp(&other.path))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrucibleType {
    Basic,
    Ultra,
}

fn crucible_dijkstra(
    grid: &Grid<u32>,
    start: Position,
    goal: Position,
    crucible_type: CrucibleType,
) -> Option<PathState> {
    let mut dists: Grid<HashMap<(Direction, u8), u64>> = Grid(
        (0..grid.len())
            .map(|_| (0..grid[0].len()).map(|_| HashMap::new()).collect())
            .collect(),
    );

    let mut heap = BinaryHeap::new();
    heap.push(PathState {
        dist: 0,
        pos: start,
        last_move: (Direction::North, 0),
        path: Vec::new(),
    });
    dists[start.1][start.0].insert((Direction::North, 0), 0);

    while let Some(PathState {
        dist,
        pos,
        last_move,
        path,
    }) = heap.pop()
    {
        if pos == goal {
            return Some(PathState {
                dist,
                pos,
                last_move,
                path: path.clone(),
            });
        }

        if dist
            > dists[pos.1][pos.0]
                .get(&last_move)
                .copied()
                .unwrap_or(u64::MAX)
        {
            continue;
        }

        for direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if last_move.1 != 0
                && (direction == last_move.0
                    || direction.turned(RelativeDirection::Backward) == last_move.0)
            {
                continue;
            }

            let unit_vector = direction.unit_vector();
            let mut potential_path = path.clone();
            let mut prev_pos = pos;
            let mut prev_dist = dist;

            let block_range = if crucible_type == CrucibleType::Ultra {
                1..=10
            } else {
                1..=3
            };

            for blocks in block_range {
                if let Ok(potential_pos) = prev_pos + unit_vector {
                    if let Some(potential_dist_delta) = grid.get_pos(potential_pos) {
                        potential_path.push((prev_pos, direction));
                        prev_pos = potential_pos;
                        prev_dist += *potential_dist_delta as u64;
                        let next = PathState {
                            pos: potential_pos,
                            dist: prev_dist,
                            last_move: (direction, blocks as u8),
                            path: potential_path.clone(),
                        };

                        if next.dist
                            < dists[next.pos.1][next.pos.0]
                                .get(&next.last_move)
                                .copied()
                                .unwrap_or(u64::MAX)
                            && (crucible_type == CrucibleType::Basic || blocks >= 4)
                        {
                            dists[next.pos.1][next.pos.0].insert(next.last_move, next.dist);
                            heap.push(next);
                        }
                    }
                }
            }
        }
    }

    None
}
