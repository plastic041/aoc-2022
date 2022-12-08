pub fn part_one(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");

    let cargo_string = split.next().unwrap();

    let line_str_len = cargo_string.lines().clone().next().unwrap().len();
    let line_len = (line_str_len + 1) / 4;

    println!("Line length: {}", line_len);

    let mut cargos: Vec<Vec<char>> = vec![vec![]; line_len];

    'outer: for line in cargo_string.lines() {
        'inner: for i in (0..line_str_len).step_by(4) {
            let c = line.chars().nth(i + 1).unwrap();

            if c == '1' {
                break 'outer;
            } else if c == ' ' {
                continue 'inner;
            }

            cargos[i / 4].push(c);
        }
    }

    cargos = cargos
        .iter()
        .map(|x| x.iter().rev().cloned().collect())
        .collect();

    let commands = split.next().unwrap().lines();

    for command in commands {
        let line = command
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");

        let mut split = line.split(' ');
        let move_count = split.next().unwrap().parse::<usize>().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..move_count {
            let c = cargos[from].pop().unwrap();
            cargos[to].push(c);
        }
    }

    let mut lasts: Vec<char> = vec![];
    for cargo in cargos {
        lasts.push(*cargo.last().unwrap());
    }

    Some(lasts.into_iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");

    let cargo_string = split.next().unwrap();

    let line_str_len = cargo_string.lines().clone().next().unwrap().len();
    let line_len = (line_str_len + 1) / 4;

    println!("Line length: {}", line_len);

    let mut cargos: Vec<Vec<char>> = vec![vec![]; line_len];

    'outer: for line in cargo_string.lines() {
        'inner: for i in (0..line_str_len).step_by(4) {
            let c = line.chars().nth(i + 1).unwrap();

            if c == '1' {
                break 'outer;
            } else if c == ' ' {
                continue 'inner;
            }

            cargos[i / 4].push(c);
        }
    }

    cargos = cargos
        .iter()
        .map(|x| x.iter().rev().cloned().collect())
        .collect();

    let commands = split.next().unwrap().lines();

    for command in commands {
        let line = command
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "");

        let mut split = line.split(' ');
        let move_count = split.next().unwrap().parse::<usize>().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut cargos_to_move: Vec<char> = vec![];
        for _ in 0..move_count {
            let c = cargos[from].pop().unwrap();
            cargos_to_move.push(c);
        }
        cargos_to_move = cargos_to_move.iter().rev().cloned().collect();

        for c in cargos_to_move {
            cargos[to].push(c);
        }
    }

    let mut lasts: Vec<char> = vec![];
    for cargo in cargos {
        lasts.push(*cargo.last().unwrap());
    }

    Some(lasts.into_iter().collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
