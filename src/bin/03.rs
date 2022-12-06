use advent_of_code::helpers::alphabet_score;

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut letters_common: Vec<char> = vec![];

    'outer: for line in lines {
        let len = line.len();
        let half_len = len / 2;
        let first = &line[0..half_len];
        let second = &line[half_len..len];

        for (_, c) in first.chars().enumerate() {
            if second.contains(c) {
                letters_common.push(c);
                continue 'outer;
            }
        }
    }

    let mut sum = 0;
    for c in letters_common {
        sum += alphabet_score(c);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();
    let len = lines.clone().count();

    let mut badges: Vec<char> = vec![];

    'outer: for i in (0..len).step_by(3) {
        let first = lines.clone().nth(i).unwrap();
        let second = lines.clone().nth(i + 1).unwrap();
        let third = lines.clone().nth(i + 2).unwrap();

        for (_, c) in first.chars().enumerate() {
            if second.contains(c) && third.contains(c) {
                badges.push(c);
                continue 'outer;
            }
        }
    }

    let mut sum = 0;
    for c in badges {
        sum += alphabet_score(c);
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
