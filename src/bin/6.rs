use std::io;

fn main() {
    let mut lines = io::stdin().lines().map(|line| {
        let line = line.unwrap();
        (
            line.split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>(),
            line.split(':')
                .skip(1)
                .next()
                .unwrap()
                .replace(" ", "")
                .parse::<u64>()
                .unwrap(),
        )
    });
    let (times, long_time) = lines.next().unwrap();
    let (records, long_record) = lines.next().unwrap();

    let part1 = times
        .iter()
        .zip(&records)
        .map(|(&time, &record_dist)| simulate_records(time, record_dist))
        .fold(1, |acc, x| acc * x);

    let part2 = simulate_records(long_time, long_record);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn simulate_records(time: u64, record_dist: u64) -> u64 {
    let mut wins = 0;
    for t in 0..time {
        let dist = t * (time - t);
        if dist > record_dist {
            wins += 1;
        }
    }

    wins
}
