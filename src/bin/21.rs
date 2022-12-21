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

fn get_equation(monkeys: &HashMap<String, Monkey>, name: &str) -> String {
    let monkey = monkeys.get(name).unwrap();
    match monkey.name.as_str() {
        "root" => match &monkey.job {
            Job::Yell(_) => panic!("Root is not a yell"),
            Job::MathThenYell(math) => {
                let other = get_equation(monkeys, &math.other_name);
                let other_2 = get_equation(monkeys, &math.other_name_2);

                format!("{} - {} = 0", other, other_2)
            }
        },
        "humn" => "x".to_string(),
        _ => match &monkey.job {
            Job::Yell(yell) => yell.to_string(),
            Job::MathThenYell(math) => {
                let other = get_equation(monkeys, &math.other_name);
                let other_2 = get_equation(monkeys, &math.other_name_2);

                let operator = match math.math {
                    Operation::Add => "+",
                    Operation::Multiply => "*",
                    Operation::Minus => "-",
                    Operation::Divide => "/",
                };

                if other.contains("x") || other_2.contains("x") {
                    return format!("({} {} {})", other, operator, other_2);
                } else {
                    let result = match math.math {
                        Operation::Add => {
                            other.parse::<i64>().unwrap() + other_2.parse::<i64>().unwrap()
                        }
                        Operation::Multiply => {
                            other.parse::<i64>().unwrap() * other_2.parse::<i64>().unwrap()
                        }
                        Operation::Minus => {
                            other.parse::<i64>().unwrap() - other_2.parse::<i64>().unwrap()
                        }
                        Operation::Divide => {
                            other.parse::<i64>().unwrap() / other_2.parse::<i64>().unwrap()
                        }
                    };
                    return result.to_string();
                }
            }
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Term {
    Number(i64),
    Variable,
    Term(Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
struct Expression {
    left: Term,
    right: Term,
    operator: Operation,
}

fn get_equation_2(monkeys: &HashMap<String, Monkey>, name: &str) -> Term {
    let monkey = monkeys.get(name).unwrap();
    match monkey.name.as_str() {
        "root" => match &monkey.job {
            Job::Yell(_) => panic!("Root is not a yell"),
            Job::MathThenYell(math) => {
                let other = get_equation_2(monkeys, &math.other_name);
                let other_2 = get_equation_2(monkeys, &math.other_name_2);

                Term::Term(Box::new(Expression {
                    left: other,
                    right: other_2,
                    operator: math.math.clone(),
                }))
            }
        },
        "humn" => Term::Variable,
        _ => match &monkey.job {
            Job::Yell(yell) => Term::Number(*yell),
            Job::MathThenYell(math) => {
                let other = get_equation_2(monkeys, &math.other_name);
                let other_2 = get_equation_2(monkeys, &math.other_name_2);

                match (&other, &other_2) {
                    (Term::Variable, Term::Variable) => panic!("Can't have two variables"),
                    (Term::Number(a), Term::Number(b)) => {
                        let result = match math.math {
                            Operation::Add => a + b,
                            Operation::Multiply => a * b,
                            Operation::Minus => a - b,
                            Operation::Divide => a / b,
                        };
                        return Term::Number(result);
                    }
                    (_, _) => {
                        return Term::Term(Box::new(Expression {
                            left: other,
                            right: other_2,
                            operator: math.math.clone(),
                        }));
                    }
                }
            }
        },
    }
}

fn op(operation: &Operation, right: &mut i64, a: &i64) {
    match operation {
        Operation::Add => *right -= *a,
        Operation::Multiply => *right /= *a,
        Operation::Minus => *right += *a,
        Operation::Divide => *right *= *a,
    }
}

fn solve_equation(equation: &mut Term) -> i64 {
    let mut left = equation;
    let mut right = 0;

    while let Term::Term(e) = left {
        let left_term = &mut e.left;
        let right_term = &mut e.right;

        match (&left_term, &right_term) {
            (Term::Variable, Term::Variable) => panic!("Can't have two variables"),
            (Term::Number(_), Term::Number(_)) => todo!(),

            (Term::Variable, Term::Number(a)) | (Term::Term(_), Term::Number(a)) => {
                op(&e.operator, &mut right, a);
                left = left_term;
            }
            (Term::Number(_), Term::Variable) => todo!(),
            (Term::Number(_), Term::Term(_)) => todo!(),
            (Term::Variable, Term::Term(_)) => todo!(),
            (Term::Term(_), Term::Variable) => todo!(),
            (Term::Term(_), Term::Term(_)) => todo!(),
        }
    }

    0
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

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in input.lines() {
        let monkey = Monkey::parse_line(line);
        monkeys.insert(monkey.name.clone(), monkey);
    }

    // Found this equation by hand
    let equation = get_equation_2(&monkeys, "root");

    println!("{:?}", equation);

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
