pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut pairs_count: i32 = 0;

    for line in lines {
        let mut split = line.split(',');
        let first = split.next().unwrap();
        let second = split.next().unwrap();

        let mut firsts = first.split('-');
        let first_start = firsts.next().unwrap().parse::<i32>().unwrap();
        let first_end = firsts.next().unwrap().parse::<i32>().unwrap();

        let mut seconds = second.split('-');
        let second_start = seconds.next().unwrap().parse::<i32>().unwrap();
        let second_end = seconds.next().unwrap().parse::<i32>().unwrap();

        if (first_start <= second_start && second_end <= first_end)
            || (second_start <= first_start && first_end <= second_end)
        {
            pairs_count += 1;
        }
    }

    Some(pairs_count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut pairs_count: i32 = 0;

    for line in lines {
        let mut split = line.split(',');
        let first = split.next().unwrap();
        let second = split.next().unwrap();

        let mut firsts = first.split('-');
        let first_start = firsts.next().unwrap().parse::<i32>().unwrap();
        let first_end = firsts.next().unwrap().parse::<i32>().unwrap();

        let mut seconds = second.split('-');
        let second_start = seconds.next().unwrap().parse::<i32>().unwrap();
        let second_end = seconds.next().unwrap().parse::<i32>().unwrap();

        if (first_start <= second_start && second_start <= first_end)
            || (first_start <= second_end && second_end <= first_end)
            || (second_start <= first_start && first_start <= second_end)
            || (second_start <= first_end && first_end <= second_end)
        {
            pairs_count += 1;
        }
    }

    Some(pairs_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
