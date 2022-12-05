use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let rations = input.split("\n\n");

    let max_calories = rations
        .map(|ration| {
            let calories = ration
                .split('\n')
                .map(|line| {
                    line.parse::<i32>()
                        .unwrap_or_else(|_| panic!("line {} is not a number", line))
                })
                .sum();

            calories
        })
        .collect::<Vec<i32>>()
        .into_iter()
        .reduce(i32::max)
        .unwrap_or_else(|| panic!("'Calorieses' is empty"));

    Some(max_calories)
}

pub fn part_two(input: &str) -> Option<i32> {
    let rations = input.split("\n\n");

    let top_3_calorieses_sum: i32 = rations
        .map(|ration| {
            let calories: i32 = ration
                .split('\n')
                .map(|line| {
                    line.parse::<i32>()
                        .unwrap_or_else(|_| panic!("line {} is not a number", line))
                })
                .sum();

            calories
        })
        .sorted_unstable()
        .rev()
        .take(3)
        .sum();

    Some(top_3_calorieses_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
