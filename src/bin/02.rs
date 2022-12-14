pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();

    fn rps_score(opponent: &str, you: &str) -> Option<i32> {
        let shape_score = match you {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Invalid shape"),
        };

        let match_score = match you {
            // rock
            "X" => match opponent {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                _ => panic!("Invalid shape"),
            },
            // paper
            "Y" => match opponent {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => panic!("Invalid shape"),
            },
            // scissors
            "Z" => match opponent {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => panic!("Invalid shape"),
            },
            _ => panic!("Invalid shape"),
        };

        Some(shape_score + match_score)
    }

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
            "X" => 3, // you lose by scissors 0 + 3
            "Y" => 4, // you draw by rock 3 + 1
            "Z" => 8, // you win by paper 6 + 2
            _ => panic!("Invalid shape"),
        },
        // paper
        "B" => match you {
            "X" => 1, // you lose by rock 0 + 1
            "Y" => 5, // you draw by paper 3 + 2
            "Z" => 9, // you win by scissors 6 + 3
            _ => panic!("Invalid shape"),
        },
        // scissors
        "C" => match you {
            "X" => 2, // you lose by paper 0 + 2
            "Y" => 6, // you draw by scissors 3 + 3
            "Z" => 7, // you win by rock 6 + 1
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
