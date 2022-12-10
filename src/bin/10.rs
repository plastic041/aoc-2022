fn calc(cycles: u32, register: i32) -> i32 {
    (cycles as i32) * register
}

fn should_calc(cycles: u32) -> bool {
    cycles >= 20 && (cycles - 20) % 40 == 0
}

struct Cpu {
    cycles: i32,
    register: i32,
    crt_position: i32,

    crt: Vec<char>,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cycles: 1,
            register: 1,
            crt_position: 0,

            crt: vec![],
        }
    }

    fn draw(&mut self) {
        if self.crt_position == self.register - 1
            || self.crt_position == self.register
            || self.crt_position == self.register + 1
        {
            self.crt.push('#');
        } else {
            self.crt.push('.');
        }

        self.crt_position += 1;

        if self.cycles % 40 == 0 {
            println!("{}", self.crt.iter().collect::<String>());
            self.crt.clear();
            self.crt_position = 0;
        }
    }

    fn tick(&mut self) {
        self.cycles += 1;
    }

    fn update(&mut self, value: i32) {
        self.register += value;
    }
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

    let mut cpu = Cpu::new();

    for line in lines {
        if line == "noop" {
            cpu.draw();
            cpu.tick();
        } else {
            let mut split = line.split_whitespace();
            let _ = split.next();
            let value = split.next().unwrap().parse::<i32>().unwrap();

            cpu.draw();
            cpu.tick();

            cpu.draw();
            cpu.tick();
            cpu.update(value);
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
