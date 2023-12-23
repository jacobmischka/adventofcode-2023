use std::{
    collections::{HashMap, VecDeque},
    io,
    str::FromStr,
};

fn main() {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules: HashMap<String, Module> = io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                None
            } else {
                let module = Module::from_str(&line).unwrap();
                for output in &module.outputs {
                    inputs
                        .entry(output.to_string())
                        .or_default()
                        .push(module.label.clone());
                }

                Some((module.label.clone(), module))
            }
        })
        .collect();

    for module in modules.values_mut() {
        if let ModuleType::Conjunction { input_memory } = &mut module.module_type {
            *input_memory = inputs
                .remove(&module.label)
                .unwrap_or_default()
                .into_iter()
                .map(|i| (i, Pulse::Low))
                .collect();
        }
    }

    let mut all_pulses: HashMap<Pulse, u64> = HashMap::new();
    let mut pulses: VecDeque<(String, String, Pulse)> = VecDeque::new();

    let mut i = 0;
    loop {
        if i == 1000 {
            let part1 = all_pulses.get(&Pulse::Low).copied().unwrap_or_default()
                * all_pulses.get(&Pulse::High).copied().unwrap_or_default();
            println!("Part 1: {part1}");
        }

        pulses.push_back(("broadcaster".to_string(), "".to_string(), Pulse::Low));
        i += 1;
        while let Some((target, from, pulse)) = pulses.pop_front() {
            if target == "rx" && pulse == Pulse::Low {
                println!("Part 2: {i}");
                if i > 1000 {
                    return;
                }
            }

            if i <= 1000 {
                *all_pulses.entry(pulse).or_default() += 1;
            }

            if let Some(target_module) = modules.get_mut(&target) {
                if let Some(output_pulse) = target_module.receive_pulse(from, pulse) {
                    for dest in &target_module.outputs {
                        pulses.push_back((dest.clone(), target_module.label.clone(), output_pulse));
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Module {
    label: String,
    outputs: Vec<String>,
    module_type: ModuleType,
}

impl Module {
    fn receive_pulse(&mut self, input: String, pulse: Pulse) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::FlipFlop { on } => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *on = !*on;
                    if *on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
            },
            ModuleType::Conjunction { input_memory } => {
                let entry = input_memory.entry(input).or_insert(Pulse::Low);
                *entry = pulse;
                if input_memory.values().all(|p| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleType::Broadcast => Some(pulse),
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop {
        on: bool,
    },
    Conjunction {
        input_memory: HashMap<String, Pulse>,
    },
    Broadcast,
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(" -> ");
        let label = halves.next().ok_or_else(|| format!("invalid module {s}"))?;
        let outputs: Vec<String> = halves
            .next()
            .ok_or_else(|| format!("invalid module {s}"))?
            .split(", ")
            .map(String::from)
            .collect();
        if label == "broadcaster" {
            Ok(Module {
                label: label.to_string(),
                outputs,
                module_type: ModuleType::Broadcast,
            })
        } else if label.starts_with('%') {
            let label = label[1..].to_string();
            Ok(Module {
                label,
                outputs,
                module_type: ModuleType::FlipFlop { on: false },
            })
        } else if label.starts_with('&') {
            let label = label[1..].to_string();
            Ok(Module {
                label,
                module_type: ModuleType::Conjunction {
                    input_memory: HashMap::new(),
                },
                outputs,
            })
        } else {
            Err(format!("invalid module label {label}"))
        }
    }
}
