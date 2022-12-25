use std::collections::HashMap;

fn get_characters() -> HashMap<char, i64> {
    let mut characters = HashMap::new();
    characters.insert('2', 2);
    characters.insert('1', 1);
    characters.insert('0', 0);
    characters.insert('-', -1);
    characters.insert('=', -2);
    characters
}

fn calculate_score(snafu: &[char]) -> i64 {
    let characters = get_characters();
    let mut score = 0;
    for (i, character) in snafu.iter().rev().enumerate() {
        let multiplier = 5_i64.pow(i as u32);

        score += multiplier * characters.get(character).unwrap();
    }
    score
}

pub fn part_one(input: &str) -> Option<i64> {
    let snafus = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let snafu_scores = snafus
        .iter()
        .map(|snafu| calculate_score(&snafu.to_vec()))
        .collect::<Vec<i64>>();

    Some(snafu_scores.iter().sum::<i64>())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(4890));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
