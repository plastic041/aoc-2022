use advent_of_code::helpers::rps_score;

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();
    let score: i32 = lines
        .map(|line| {
            let mut matches = line.split_whitespace();
            let opponent = matches.next().unwrap();
            let you = matches.next().unwrap();

            rps_score(opponent, you).unwrap()
        })
        .sum();

    Some(score)
}

/// X: lose, Y: draw, Z: win
/// A: Rock, B: Paper, C: Scissors
fn calc(opponent: &str, you: &str) -> Option<i32> {
    let score = match opponent {
        // rock
        "A" => match you {
            "X" => 0 + 3, // you lose by scissors
            "Y" => 3 + 1, // you draw by rock
            "Z" => 6 + 2, // you win by paper
            _ => panic!("Invalid shape"),
        },
        // paper
        "B" => match you {
            "X" => 0 + 1, // you lose by rock
            "Y" => 3 + 2, // you draw by paper
            "Z" => 6 + 3, // you win by scissors
            _ => panic!("Invalid shape"),
        },
        // scissors
        "C" => match you {
            "X" => 0 + 2, // you lose by paper
            "Y" => 3 + 3, // you draw by scissors
            "Z" => 6 + 1, // you win by rock
            _ => panic!("Invalid shape"),
        },
        _ => panic!("Invalid shape"),
    };

    Some(score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();

    let score: i32 = lines
        .map(|line| {
            let mut matches = line.split_whitespace();
            let opponent = matches.next().unwrap();
            let you = matches.next().unwrap();

            calc(opponent, you).unwrap()
        })
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
