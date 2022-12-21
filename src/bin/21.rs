#[derive(Debug)]
enum Job {
    Yell(i64),
    MathThenYell(Math),
}

#[derive(Debug)]
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
            return Monkey {
                name,
                job: Job::Yell(yell),
            };
        } else {
            let mut parts = job.split(" ");
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

            return Monkey {
                name,
                job: Job::MathThenYell(Math {
                    other_name,
                    math,
                    other_name_2,
                }),
            };
        }
    }
}

fn get_number(monkeys: &[Monkey], name: &str) -> i64 {
    let monkey = monkeys.iter().find(|m| m.name == name).unwrap();
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

fn check_equal(monkeys: &[Monkey], name: &str) -> i64 {
    let monkey = monkeys.iter().find(|m| m.name == name).unwrap();

    match monkey.name.as_str() {
        "root" => match &monkey.job {
            Job::Yell(_) => panic!("Root should not yell"),
            Job::MathThenYell(math) => {
                let other = get_number(monkeys, &math.other_name);
                let other_2 = get_number(monkeys, &math.other_name_2);

                if other == other_2 {
                    -1
                } else {
                    -2
                }
            }
        },
        _ => match &monkey.job {
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
        },
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = input
        .lines()
        .map(|line| Monkey::parse_line(line))
        .collect::<Vec<_>>();

    let number = get_number(&monkeys, "root");

    Some(number as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // let mut monkeys = input
    //     .lines()
    //     .map(|line| Monkey::parse_line(line))
    //     .collect::<Vec<_>>();

    // let mut humn_yell = 0;

    // // change the humn job to yell
    // for monkey in &mut monkeys {
    //     if monkey.name == "humn" {
    //         monkey.job = Job::Yell(humn_yell);
    //     }
    // }

    // loop {
    //     let number = check_equal(&monkeys, "root");

    //     if humn_yell % 1000 == 0 {
    //         println!("humm_yell: {}", humn_yell);
    //     }

    //     if number == -1 {
    //         break;
    //     } else if number == -2 {
    //         humn_yell += 1;
    //         for monkey in &mut monkeys {
    //             if monkey.name == "humn" {
    //                 monkey.job = Job::Yell(humn_yell);
    //             }
    //         }
    //     }
    // }

    // Some(humn_yell as u64)
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
        assert_eq!(part_two(&input), Some(301));
    }
}
