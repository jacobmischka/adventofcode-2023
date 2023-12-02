use std::io;

const DIGIT_WORDS: &[(&str, u32)] = &[
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let mut first_char: Option<(usize, u32)> = None;
        let mut last_char: Option<(usize, u32)> = None;

        for (i, c) in line.chars().enumerate() {
            match c.to_digit(10) {
                None => {}
                Some(d) => {
                    match first_char {
                        Some((prev_index, _)) => {
                            if i < prev_index {
                                first_char = Some((i, d));
                            }
                        }
                        None => {
                            first_char = Some((i, d));
                        }
                    }

                    match last_char {
                        Some((prev_index, _)) => {
                            if i > prev_index {
                                last_char = Some((i, d));
                            }
                        }
                        None => {
                            last_char = Some((i, d));
                        }
                    }
                }
            }
        }
        sum_part1 += first_char.unwrap_or_default().1 * 10 + last_char.unwrap_or_default().1;

        for i in 0..line.len() {
            for &(digit_str, d) in DIGIT_WORDS {
                if line.get(i..(i + digit_str.len())) == Some(digit_str) {
                    match first_char {
                        Some((prev_index, _)) => {
                            if i < prev_index {
                                first_char = Some((i, d));
                            }
                        }
                        None => {
                            first_char = Some((i, d));
                        }
                    }

                    match last_char {
                        Some((prev_index, _)) => {
                            if i > prev_index {
                                last_char = Some((i, d));
                            }
                        }
                        None => {
                            last_char = Some((i, d));
                        }
                    }
                }
            }
        }
        sum_part2 += first_char.unwrap_or_default().1 * 10 + last_char.unwrap_or_default().1;
    }

    println!("Part 1: {sum_part1}");
    println!("Part 2: {sum_part2}");
}
