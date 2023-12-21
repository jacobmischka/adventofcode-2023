use std::{collections::HashMap, io, str::FromStr};

fn main() {
    let mut part1 = 0;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut in_workflows = true;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if in_workflows {
            if line.is_empty() {
                in_workflows = false;
                continue;
            }

            let workflow = Workflow::from_str(&line).unwrap();
            workflows.insert(workflow.label.clone(), workflow);
        } else {
            if line.is_empty() {
                continue;
            }

            let item = Item::from_str(&line).unwrap();

            if check_item(&workflows, &item) {
                part1 += item.x + item.m + item.a + item.s;
            }
        }
    }

    println!("Part 1: {part1}");
}

fn check_item(workflows: &HashMap<String, Workflow>, item: &Item) -> bool {
    let mut label = "in";

    'outer: while let Some(w) = workflows.get(label) {
        for rule in &w.rules {
            let subject = item.get_category(rule.subject);
            match rule.operator {
                Operator::LessThan => {
                    if subject < rule.operand {
                        label = &rule.dest;
                        continue 'outer;
                    }
                }
                Operator::GreaterThan => {
                    if subject > rule.operand {
                        label = &rule.dest;
                        continue 'outer;
                    }
                }
            }
        }

        label = &w.fallback;
    }

    match label {
        "A" => true,
        "R" => false,
        l => panic!("unknown workflow label {l}"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Operator::LessThan),
            ">" => Ok(Operator::GreaterThan),
            c => Err(format!("invalid operator {c}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl FromStr for Item {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let str = str.replace(&['{', '}'], "");

        let mut x: Option<u32> = None;
        let mut m: Option<u32> = None;
        let mut a: Option<u32> = None;
        let mut s: Option<u32> = None;

        for piece in str.split(',') {
            let mut halves = piece.split('=');
            let category = Category::from_str(
                halves
                    .next()
                    .ok_or_else(|| format!("invalid item category {piece}"))?,
            )?;
            let value = halves
                .next()
                .ok_or_else(|| format!("invalid category value {piece}"))?
                .parse()
                .map_err(|_| format!("invalid item value {str}"))?;
            match category {
                Category::ExtremelyCoolLooking => {
                    x = Some(value);
                }
                Category::Musical => {
                    m = Some(value);
                }
                Category::Aerodynamic => {
                    a = Some(value);
                }
                Category::Shiny => {
                    s = Some(value);
                }
            }
        }

        Ok(Item {
            x: x.ok_or_else(|| format!("item missing x {str}"))?,
            m: m.ok_or_else(|| format!("item missing m {str}"))?,
            a: a.ok_or_else(|| format!("item missing a {str}"))?,
            s: s.ok_or_else(|| format!("item missing s {str}"))?,
        })
    }
}

impl Item {
    fn get_category(&self, category: Category) -> u32 {
        match category {
            Category::ExtremelyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }
}

impl FromStr for Category {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Category::ExtremelyCoolLooking),
            "m" => Ok(Category::Musical),
            "a" => Ok(Category::Aerodynamic),
            "s" => Ok(Category::Shiny),
            s => Err(format!("invalid category {s}")),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    subject: Category,
    operator: Operator,
    operand: u32,
    dest: String,
}

#[derive(Debug, Clone)]
struct Workflow {
    label: String,
    rules: Vec<Rule>,
    fallback: String,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(':');
        let comparison = halves.next().ok_or_else(|| format!("invalid rule {s}"))?;
        let dest = halves
            .next()
            .ok_or_else(|| format!("invalid rule {s}"))?
            .to_string();
        let subject = Category::from_str(&comparison[..1])?;
        let operator = Operator::from_str(&comparison[1..2])?;
        let operand = comparison[2..]
            .parse()
            .map_err(|_| format!("invalid operand {s}"))?;

        Ok(Rule {
            subject,
            operator,
            operand,
            dest,
        })
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split('{');
        let label = pieces
            .next()
            .ok_or_else(|| format!("invalid workflow {s}"))?
            .to_string();
        let rules_str = pieces
            .next()
            .ok_or_else(|| format!("invalid workflow {s}"))?
            .replace('}', "");
        let mut fallback: Option<String> = None;

        let rules = rules_str
            .split(',')
            .filter_map(|rule_str| {
                let rule = Rule::from_str(rule_str).ok();

                if rule.is_none() {
                    fallback = Some(rule_str.to_string());
                }

                rule
            })
            .collect();

        if let Some(fallback) = fallback {
            Ok(Workflow {
                label,
                rules,
                fallback,
            })
        } else {
            Err(format!("invalid workflow {s}, no fallback"))
        }
    }
}
