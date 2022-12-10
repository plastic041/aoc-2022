fn calc(cycles: u32, register: i32) -> i32 {
    (cycles as i32) * register
}

fn should_calc(cycles: u32) -> bool {
    cycles >= 20 && (cycles - 20) % 40 == 0
}

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut cycles: u32 = 1;
    let mut register: i32 = 1;

    let mut signal_strength: i32 = 0;

    for line in lines {
        if line == "noop" {
            cycles += 1;
            if should_calc(cycles) {
                signal_strength += calc(cycles, register);
            }
        } else {
            let mut split = line.split_whitespace();
            let _ = split.next();
            let value = split.next().unwrap().parse::<i32>().unwrap();

            cycles += 1;
            if should_calc(cycles) {
                signal_strength += calc(cycles, register);
            }

            cycles += 1;
            register += value;
            if should_calc(cycles) {
                signal_strength += calc(cycles, register);
            }
        }
    }

    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut cycles: i32 = 1;
    let mut register: i32 = 1;
    let mut crt_position: i32 = 0;

    let mut crt: Vec<String> = vec![];

    for line in lines {
        if line == "noop" {
            if crt_position == register - 1
                || crt_position == register
                || crt_position == register + 1
            {
                crt.push("#".to_string());
            } else {
                crt.push(".".to_string());
            }
            crt_position += 1;
            if cycles % 40 == 0 {
                println!("{}", crt.join(""));
                crt.clear();
                crt_position = 0;
            }

            cycles += 1;
        } else {
            let mut split = line.split_whitespace();
            let _ = split.next();
            let value = split.next().unwrap().parse::<i32>().unwrap();

            if crt_position == register - 1
                || crt_position == register
                || crt_position == register + 1
            {
                crt.push("#".to_string());
            } else {
                crt.push(".".to_string());
            }
            crt_position += 1;
            if cycles % 40 == 0 {
                println!("{}", crt.join(""));
                crt.clear();
                crt_position = 0;
            }

            cycles += 1;

            if crt_position == register - 1
                || crt_position == register
                || crt_position == register + 1
            {
                crt.push("#".to_string());
            } else {
                crt.push(".".to_string());
            }
            crt_position += 1;
            if cycles % 40 == 0 {
                println!("{}", crt.join(""));
                crt.clear();
                crt_position = 0;
            }
            cycles += 1;

            register += value;
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
