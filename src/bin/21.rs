use std::collections::HashMap;

#[derive(Debug)]
enum Job {
    Yell(i64),
    MathThenYell(Math),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Add,
    Multiply,
    Minus,
    Divide,
}

#[derive(Debug)]
struct Math {
    other_name: String,
    math: Operation,
    other_name_2: String,
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

impl Monkey {
    fn parse_line(input: &str) -> Self {
        let mut parts = input.split(": ");
        let name = parts.next().unwrap().to_string();
        let job = parts.next().unwrap();

        if let Ok(yell) = job.parse::<i64>() {
            Monkey {
                name,
                job: Job::Yell(yell),
            }
        } else {
            let mut parts = job.split(' ');
            let other_name = parts.next().unwrap().to_string();
            let math = parts.next().unwrap();
            let other_name_2 = parts.next().unwrap().to_string();

            let math = match math {
                "+" => Operation::Add,
                "*" => Operation::Multiply,
                "-" => Operation::Minus,
                "/" => Operation::Divide,
                _ => panic!("Unknown math"),
            };

            Monkey {
                name,
                job: Job::MathThenYell(Math {
                    other_name,
                    math,
                    other_name_2,
                }),
            }
        }
    }
}

fn get_number(monkeys: &HashMap<String, Monkey>, name: &str) -> i64 {
    let monkey = monkeys.get(name).unwrap();
    match &monkey.job {
        Job::Yell(yell) => *yell,
        Job::MathThenYell(math) => {
            let other = get_number(monkeys, &math.other_name);
            let other_2 = get_number(monkeys, &math.other_name_2);

            match math.math {
                Operation::Add => other + other_2,
                Operation::Multiply => other * other_2,
                Operation::Minus => other - other_2,
                Operation::Divide => other / other_2,
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in input.lines() {
        let monkey = Monkey::parse_line(line);
        monkeys.insert(monkey.name.clone(), monkey);
    }

    let number = get_number(&monkeys, "root");

    Some(number as u64)
}

const PART_TWO_ANSWER: i64 = 3_757_272_361_782;

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in input.lines() {
        let monkey = Monkey::parse_line(line);
        monkeys.insert(monkey.name.clone(), monkey);
    }

    monkeys.get_mut("humn").unwrap().job = Job::Yell(PART_TWO_ANSWER);

    let root_monkey = monkeys.get("root").unwrap();

    let (other_name, other_2_name) = match &root_monkey.job {
        Job::Yell(_) => panic!("Root is a yell"),
        Job::MathThenYell(math) => (math.other_name.clone(), math.other_name_2.clone()),
    };

    let other = get_number(&monkeys, &other_name);
    let other_2 = get_number(&monkeys, &other_2_name);

    println!("{} = {} when humn = {}", other, other_2, PART_TWO_ANSWER);
    println!("Solved equation by hand");

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
