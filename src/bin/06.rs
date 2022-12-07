use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<i32> {
    let mut location = 0;
    let len = input.len();

    for i in 0..len {
        let first = input.chars().nth(i).unwrap();
        let second = input.chars().nth(i + 1).unwrap();
        let third = input.chars().nth(i + 2).unwrap();
        let fourth = input.chars().nth(i + 3).unwrap();

        let mut hash = HashSet::new();
        hash.insert(first);
        hash.insert(second);
        hash.insert(third);
        hash.insert(fourth);

        if hash.len() == 4 {
            location = i + 4;
            break;
        }
    }

    Some(location as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut location = 0;
    let len = input.len();

    'outer: for i in 0..len {
        let mut hash = HashSet::new();

        for cursor in 0..14 {
            let c = input.chars().nth(i + cursor).unwrap();
            hash.insert(c);
        }

        if hash.len() == 14 {
            location = i + 14;
            break 'outer;
        }
    }

    Some(location as i32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
