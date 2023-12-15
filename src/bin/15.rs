use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut part2 = 0;
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let part1 = s.split(',').fold(0, |part1, operation| {
        let operation_character_index = operation.find(&['-', '=']).unwrap();
        let label = &operation[0..operation_character_index];
        let box_index = get_hash(label) as usize;

        match &operation[operation_character_index..operation_character_index + 1] {
            "-" => {
                boxes[box_index].retain(|lens| lens.label != label);
            }
            "=" => {
                let rest = &operation[operation_character_index + 1..].trim_end();
                let focal_length: usize = rest.parse().unwrap();

                let mut replaced = false;
                for lens in &mut boxes[box_index] {
                    if lens.label == label {
                        lens.focal_length = focal_length;
                        replaced = true;
                        break;
                    }
                }

                if !replaced {
                    boxes[box_index].push(Lens {
                        label,
                        focal_length,
                    })
                }
            }
            s => panic!("unrecognized operation character {s}"),
        }

        part1 + get_hash(operation)
    });

    for (box_index, b) in boxes.iter().enumerate() {
        for (slot_index, lens) in b.iter().enumerate() {
            part2 += (box_index + 1) * (slot_index + 1) * lens.focal_length;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn get_hash(s: &str) -> u32 {
    let mut current_value = 0;
    for c in s.chars() {
        if c.is_ascii_whitespace() {
            continue;
        }

        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}
