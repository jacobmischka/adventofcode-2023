use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut card_counts: HashMap<usize, u32> = HashMap::new();

    for (i, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let mut sections = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(" | ")
            .map(|section| section.split_ascii_whitespace());

        let num_cards = card_counts.get(&i).copied().unwrap_or(1);

        let winning: HashSet<_> = sections.next().unwrap().collect();

        let mut num_matches = 0;
        for your_num in sections.next().unwrap() {
            if winning.contains(&your_num) {
                num_matches += 1;
            }
        }

        part2 += num_cards;

        if num_matches > 0 {
            part1 += 2_u32.pow(num_matches - 1);

            for j in (i + 1)..=(i + num_matches as usize) {
                let entry = card_counts.entry(j).or_insert(1);
                *entry += num_cards;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
