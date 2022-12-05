/// your shapes
/// X: rock, Y: paper, Z: scissors
///
/// opponent shapes
/// A: rock, B: paper, C: scissors
///
/// match scores
/// draw: 3, win: 6, lose: 0
pub fn rps_score(opponent: &str, you: &str) -> Option<i32> {
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
