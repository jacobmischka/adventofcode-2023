use std::io;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut nums: Vec<Vec<i64>> = vec![line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()];

        while !nums.last().unwrap().iter().all(|n| *n == 0) {
            let last = nums.last().unwrap();
            nums.push(
                (0..(last.len() - 1))
                    .map(|i| last[i + 1] - last[i])
                    .collect(),
            );
        }

        for i in (1..nums.len()).rev() {
            let new_beginning = nums[i - 1].first().unwrap() - nums[i].first().unwrap();
            // shifting here isn't performant but it's fast enough to not matter
            nums[i - 1].insert(0, new_beginning);
            let new_end = nums[i - 1].last().unwrap() + nums[i].last().unwrap();
            nums[i - 1].push(new_end);
        }

        part1 += nums[0].last().unwrap();
        part2 += nums[0].first().unwrap();
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
