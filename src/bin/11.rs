use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Multiply,
    Square,
}

#[derive(Copy, Clone, Debug)]
struct Item {
    worry_level: u64,
}

#[derive(Clone, Debug)]
struct Monkey {
    id: u32,
    inspected_count: u32,
    items: Vec<Item>,
    operation: Option<Operation>,
    operation_value: Option<u64>,
    test_divider: Option<u64>,
    throw_to_if_true: Option<u32>,
    throw_to_if_false: Option<u32>,
}

impl Monkey {
    fn new(id: u32) -> Monkey {
        Monkey {
            id,
            inspected_count: 0,
            items: Vec::new(),
            operation: None,
            operation_value: None,
            test_divider: None,
            throw_to_if_true: None,
            throw_to_if_false: None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| line.trim());

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in lines {
        if line.starts_with("Monkey") {
            // Monkey u32:
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            // remove :
            let id = parts
                .next()
                .unwrap()
                .trim_end_matches(':')
                .parse::<u32>()
                .unwrap();

            monkeys.push(Monkey::new(id));
        } else if line.starts_with("Starting items") {
            let mut parts = line.split(':');
            let _ = parts.next();
            let items = parts.next().unwrap().trim();

            println!("Items: {}", items);
            let monkey = monkeys.last_mut().unwrap();
            monkey.items = items
                .split(',')
                .map(|item| item.trim().parse().unwrap())
                .map(|worry_level| Item { worry_level })
                .collect();
        } else if line.starts_with("Operation") {
            // Operation: new = old * old

            let mut parts = line.split('=');
            let _ = parts.next();

            let operation = parts.next().unwrap().trim();

            if operation == "old * old" {
                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Some(Operation::Square);
            } else if operation.starts_with("old * ") {
                // old * some u32
                let mut parts = operation.split('*');
                let _ = parts.next();
                let multiplier = parts.next().unwrap().trim().parse().unwrap();

                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Some(Operation::Multiply);
                monkey.operation_value = Some(multiplier);
            } else if operation.starts_with("old + ") {
                // old + some u32
                let mut parts = operation.split('+');
                let _ = parts.next();
                let adder = parts.next().unwrap().trim().parse().unwrap();

                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Some(Operation::Add);
                monkey.operation_value = Some(adder);
            } else {
                panic!("Unknown operation: {}", operation);
            }
        } else if line.starts_with("Test") {
            // Test: Divisible by u32
            let mut parts = line.split("by");
            let _ = parts.next();
            let divider = parts.next().unwrap().trim().parse().unwrap();

            let monkey = monkeys.last_mut().unwrap();
            monkey.test_divider = Some(divider);
        } else if line.starts_with("If true") {
            // If true: throw to monkey u32
            let mut parts = line.split("to monkey");
            let _ = parts.next();
            let id = parts.next().unwrap().trim().parse().unwrap();

            let monkey = monkeys.last_mut().unwrap();

            monkey.throw_to_if_true = Some(id);
        } else if line.starts_with("If false") {
            // If false: throw to monkey u32
            let mut parts = line.split("to monkey");
            let _ = parts.next();
            let id = parts.next().unwrap().trim().parse::<u32>().unwrap();

            let monkey = monkeys.last_mut().unwrap();

            monkey.throw_to_if_false = Some(id);
        } else if line.is_empty() {
            // skip
        } else {
            panic!("Unknown line: {}", line);
        }
    }

    for i in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let (left, mid_right) = monkeys.split_at_mut(monkey_index);
            let (mid, right) = mid_right.split_at_mut(1);

            let monkey = &mut mid[0];
            println!("current monkey: {:?}", monkey.id);
            println!("items: {:?}", monkey.items);

            println!("Monkey {} round {}", monkey.id, i);
            for item in monkey.items.iter_mut() {
                // inspect
                println!(
                    "   Monkey inspects item {} with a worry level of {}",
                    monkey.id, item.worry_level
                );
                monkey.inspected_count += 1;

                // operate
                item.worry_level = match monkey.operation {
                    Some(Operation::Add) => item.worry_level + monkey.operation_value.unwrap(),
                    Some(Operation::Multiply) => {
                        println!(
                            "       {} * {}",
                            item.worry_level,
                            monkey.operation_value.unwrap()
                        );
                        item.worry_level * monkey.operation_value.unwrap()
                    }
                    Some(Operation::Square) => item.worry_level * item.worry_level,
                    None => panic!("No operation"),
                };
                println!(
                    "       Monkey {} operates on item {} to {}",
                    monkey.id, item.worry_level, item.worry_level
                );

                // gets bored
                item.worry_level /= 3;
                println!(
                    "       Monkey {} gets bored with item {} to {}",
                    monkey.id, item.worry_level, item.worry_level
                );

                // test
                let test_result = item.worry_level % monkey.test_divider.unwrap() as u64 == 0;
                println!(
                    "       Monkey {} tests item {} to {}",
                    monkey.id, item.worry_level, test_result
                );

                // if true
                if test_result {
                    println!(
                        "       Monkey {} throws item {} to monkey {}",
                        monkey.id,
                        item.worry_level,
                        monkey.throw_to_if_true.unwrap()
                    );

                    for other_monkey in left.iter_mut().chain(right.iter_mut()) {
                        if other_monkey.id == monkey.throw_to_if_true.unwrap() {
                            println!("other: {}", other_monkey.id);
                            // throw item
                            other_monkey.items.push(*item);
                        }
                    }
                } else {
                    println!(
                        "       Monkey {} throws item {} to monkey {}",
                        monkey.id,
                        item.worry_level,
                        monkey.throw_to_if_false.unwrap()
                    );

                    for other_monkey in left.iter_mut().chain(right.iter_mut()) {
                        if other_monkey.id == monkey.throw_to_if_false.unwrap() {
                            println!("other: {}", other_monkey.id);
                            // throw item
                            other_monkey.items.push(*item);
                        }
                    }
                }
            }
            monkey.items.clear();
        }
    }

    for monkey in monkeys.iter() {
        println!(
            "Monkey {}: count: {}, items: {:?}",
            monkey.id, monkey.inspected_count, monkey.items
        );
    }

    let mut inspected_counts = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .sorted_unstable()
        .rev();

    let first = inspected_counts.next().unwrap();
    let second = inspected_counts.next().unwrap();

    Some(first * second)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| line.trim());

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in lines {
        if line.starts_with("Monkey") {
            // Monkey u32:
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            // remove :
            let id = parts
                .next()
                .unwrap()
                .trim_end_matches(':')
                .parse::<u32>()
                .unwrap();

            monkeys.push(Monkey::new(id));
        } else if line.starts_with("Starting items") {
            let mut parts = line.split(':');
            let _ = parts.next();
            let items = parts.next().unwrap().trim();

            println!("Items: {}", items);
            let monkey = monkeys.last_mut().unwrap();
            monkey.items = items
                .split(',')
                .map(|item| item.trim().parse().unwrap())
                .map(|worry_level| Item { worry_level })
                .collect();
        } else if line.starts_with("Operation") {
            // Operation: new = old * old

            let mut parts = line.split('=');
            let _ = parts.next();

            let operation = parts.next().unwrap().trim();

            if operation == "old * old" {
                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Some(Operation::Square);
            } else if operation.starts_with("old * ") {
                // old * some u32
                let mut parts = operation.split('*');
                let _ = parts.next();
                let multiplier = parts.next().unwrap().trim().parse().unwrap();

                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Some(Operation::Multiply);
                monkey.operation_value = Some(multiplier);
            } else if operation.starts_with("old + ") {
                // old + some u32
                let mut parts = operation.split('+');
                let _ = parts.next();
                let adder = parts.next().unwrap().trim().parse().unwrap();

                let monkey = monkeys.last_mut().unwrap();
                monkey.operation = Some(Operation::Add);
                monkey.operation_value = Some(adder);
            } else {
                panic!("Unknown operation: {}", operation);
            }
        } else if line.starts_with("Test") {
            // Test: Divisible by u32
            let mut parts = line.split("by");
            let _ = parts.next();
            let divider = parts.next().unwrap().trim().parse().unwrap();

            let monkey = monkeys.last_mut().unwrap();
            monkey.test_divider = Some(divider);
        } else if line.starts_with("If true") {
            // If true: throw to monkey u32
            let mut parts = line.split("to monkey");
            let _ = parts.next();
            let id = parts.next().unwrap().trim().parse::<u32>().unwrap();

            let monkey = monkeys.last_mut().unwrap();

            monkey.throw_to_if_true = Some(id);
        } else if line.starts_with("If false") {
            // If false: throw to monkey u32
            let mut parts = line.split("to monkey");
            let _ = parts.next();
            let id = parts.next().unwrap().trim().parse::<u32>().unwrap();

            let monkey = monkeys.last_mut().unwrap();

            monkey.throw_to_if_false = Some(id);
        } else if line.is_empty() {
            // skip
        } else {
            panic!("Unknown line: {}", line);
        }
    }

    let modulus: u64 = monkeys.iter().map(|m| m.test_divider.unwrap()).product();

    for i in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let (left, mid_right) = monkeys.split_at_mut(monkey_index);
            let (mid, right) = mid_right.split_at_mut(1);

            let monkey = &mut mid[0];
            println!("current monkey: {:?}", monkey.id);
            println!("items: {:?}", monkey.items);

            println!("Monkey {} round {}", monkey.id, i);
            for item in monkey.items.iter_mut() {
                // inspect
                println!(
                    "   Monkey inspects item {} with a worry level of {}",
                    monkey.id, item.worry_level
                );
                monkey.inspected_count += 1;

                // operate
                item.worry_level = match monkey.operation {
                    Some(Operation::Add) => item.worry_level + monkey.operation_value.unwrap(),
                    Some(Operation::Multiply) => {
                        println!(
                            "       {} * {}",
                            item.worry_level,
                            monkey.operation_value.unwrap()
                        );
                        item.worry_level * monkey.operation_value.unwrap()
                    }
                    Some(Operation::Square) => item.worry_level * item.worry_level,
                    None => panic!("No operation"),
                };
                println!(
                    "       Monkey {} operates on item {} to {}",
                    monkey.id, item.worry_level, item.worry_level
                );

                // manage worry level
                item.worry_level %= modulus;

                // test
                let test_result = item.worry_level % monkey.test_divider.unwrap() as u64 == 0;
                println!(
                    "       Monkey {} tests item {} to {}",
                    monkey.id, item.worry_level, test_result
                );

                // if true
                if test_result {
                    println!(
                        "       Monkey {} throws item {} to monkey {}",
                        monkey.id,
                        item.worry_level,
                        monkey.throw_to_if_true.unwrap()
                    );

                    for other_monkey in left.iter_mut().chain(right.iter_mut()) {
                        if other_monkey.id == monkey.throw_to_if_true.unwrap() {
                            println!("other: {}", other_monkey.id);
                            // throw item
                            other_monkey.items.push(*item);
                        }
                    }
                } else {
                    println!(
                        "       Monkey {} throws item {} to monkey {}",
                        monkey.id,
                        item.worry_level,
                        monkey.throw_to_if_false.unwrap()
                    );

                    for other_monkey in left.iter_mut().chain(right.iter_mut()) {
                        if other_monkey.id == monkey.throw_to_if_false.unwrap() {
                            println!("other: {}", other_monkey.id);
                            // throw item
                            other_monkey.items.push(*item);
                        }
                    }
                }
            }
            monkey.items.clear();
        }
    }

    for monkey in monkeys.iter() {
        println!(
            "Monkey {}: count: {}, items: {:?}",
            monkey.id, monkey.inspected_count, monkey.items
        );
    }

    let mut inspected_counts = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .sorted_unstable()
        .rev();

    let first = inspected_counts.next().unwrap();
    let second = inspected_counts.next().unwrap();

    Some(first * second)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
