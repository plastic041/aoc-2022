#[derive(Debug, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn parse(input: &str) -> Option<Vec<Move>> {
        let mut moves = vec![];
        for c in input.chars() {
            match c {
                '<' => moves.push(Move::Left),
                '>' => moves.push(Move::Right),
                _ => return None,
            }
        }
        Some(moves)
    }
}

#[derive(Debug)]
struct RockShape {
    height: u32,
    rocks: Vec<Vec<bool>>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = Move::parse(input).unwrap();
    let base_rock_shapes = vec![
        vec![vec![false, false, true, true, true, true, false]],
        vec![
            vec![false, false, false, true, false, false, false],
            vec![false, false, true, true, true, false, false],
            vec![false, false, false, true, false, false, false],
        ],
        vec![
            vec![false, false, false, false, true, false, false],
            vec![false, false, false, false, true, false, false],
            vec![false, false, true, true, true, false, false],
        ],
        vec![
            vec![false, false, true, false, false, false, false],
            vec![false, false, true, false, false, false, false],
            vec![false, false, true, false, false, false, false],
            vec![false, false, true, false, false, false, false],
        ],
        vec![
            vec![false, false, true, true, false, false, false],
            vec![false, false, true, true, false, false, false],
        ],
    ];
    let mut world: Vec<Vec<bool>> = vec![];
    let mut loop_counter = 0;
    let mut move_counter = 0;

    loop {
        let current_shape = base_rock_shapes[loop_counter % 5].clone();

        let mut shape = RockShape {
            height: world.len() as u32 + 3 + current_shape.len() as u32,
            rocks: current_shape,
        };
        for _ in 0..=shape.rocks.len() + 3 {
            world.push(vec![false; 7]);
        }

        // println!("Rock {} begins falling", loop_counter + 1);
        // dbg!(shape.height, world.len());

        while can_fall(&shape, &world) {
            let current_move = &moves[move_counter % moves.len()];
            move_counter += 1;

            shape.height -= 1;
            // println!("Rock {} falls 1 unit to {}", loop_counter + 1, shape.height);

            if can_move(&shape, &world, current_move) {
                // println!("Rock {} moves {:?}", loop_counter + 1, current_move);
                shape.rocks = shape
                    .rocks
                    .iter()
                    .map(|row| match current_move {
                        Move::Left => {
                            let mut new_row = row.clone();
                            new_row.remove(0);
                            new_row.push(false);
                            new_row
                        }
                        Move::Right => {
                            let mut new_row = row.clone();
                            new_row.remove(row.len() - 1);
                            new_row.insert(0, false);
                            new_row
                        }
                    })
                    .collect();
            } else {
                // println!("Rock {} cannot move {:?}", loop_counter + 1, current_move);
            }
        }

        for (dy, row) in shape.rocks.iter().enumerate() {
            for (x, &is_rock) in row.iter().enumerate() {
                if is_rock {
                    world[shape.height as usize - dy][x] = true;
                }
            }
        }

        world = world
            .iter()
            // remove all rows that are all false
            .filter(|row| row.iter().any(|&b| b))
            .cloned()
            .collect();

        // for row in world.iter().rev() {
        //     for b in row {
        //         print!("{}", if *b { '#' } else { '.' });
        //     }
        //     println!();
        // }

        // println!();

        if loop_counter == 2021 {
            // actually 2022, but we're counting from 0
            break;
        }
        loop_counter += 1;
    }

    Some(world.len() as u32)
}

fn can_fall(shape: &RockShape, world: &[Vec<bool>]) -> bool {
    if shape.height == 0 {
        return false;
    }
    for (dy, row) in shape.rocks.iter().enumerate() {
        for (x, &is_rock) in row.iter().enumerate() {
            if is_rock && world[shape.height as usize - dy - 1][x] {
                return false;
            }
        }
    }
    true
}

fn can_move(shape: &RockShape, world: &[Vec<bool>], direction: &Move) -> bool {
    match direction {
        Move::Left => {
            if shape.rocks.iter().any(|row| row[0]) {
                false
            } else {
                for (dy, row) in shape.rocks.iter().enumerate() {
                    for (x, &is_rock) in row.iter().enumerate() {
                        if is_rock && world[shape.height as usize - dy][x - 1] {
                            return false;
                        }
                    }
                }
                true
            }
        }
        Move::Right => {
            if shape.rocks.iter().any(|row| row[row.len() - 1]) {
                false
            } else {
                for (dy, row) in shape.rocks.iter().enumerate() {
                    for (x, &is_rock) in row.iter().enumerate() {
                        if is_rock && world[shape.height as usize - dy][x + 1] {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
    // let moves = Move::parse(input).unwrap();
    // let moves_len = moves.len();
    // let base_rock_shapes = vec![
    //     vec![vec![false, false, true, true, true, true, false]],
    //     vec![
    //         vec![false, false, false, true, false, false, false],
    //         vec![false, false, true, true, true, false, false],
    //         vec![false, false, false, true, false, false, false],
    //     ],
    //     vec![
    //         vec![false, false, false, false, true, false, false],
    //         vec![false, false, false, false, true, false, false],
    //         vec![false, false, true, true, true, false, false],
    //     ],
    //     vec![
    //         vec![false, false, true, false, false, false, false],
    //         vec![false, false, true, false, false, false, false],
    //         vec![false, false, true, false, false, false, false],
    //         vec![false, false, true, false, false, false, false],
    //     ],
    //     vec![
    //         vec![false, false, true, true, false, false, false],
    //         vec![false, false, true, true, false, false, false],
    //     ],
    // ];
    // let mut world: Vec<Vec<bool>> = vec![];
    // let mut loop_counter = 0;
    // let mut move_counter = 0;

    // loop {
    //     let current_shape = base_rock_shapes[loop_counter % 5].clone();

    //     let mut shape = RockShape {
    //         height: world.len() as u32 + 3 + current_shape.len() as u32,
    //         rocks: current_shape,
    //     };
    //     for _ in 0..=shape.rocks.len() + 3 {
    //         world.push(vec![false; 7]);
    //     }

    //     while can_fall(&shape, &world) {
    //         let current_move = &moves[move_counter % moves_len];
    //         move_counter += 1;

    //         shape.height -= 1;

    //         if can_move(&shape, &world, current_move) {
    //             shape.rocks.iter_mut().for_each(|row| match current_move {
    //                 Move::Left => {
    //                     row.remove(0);
    //                     row.push(false);
    //                 }
    //                 Move::Right => {
    //                     row.remove(row.len() - 1);
    //                     row.insert(0, false);
    //                 }
    //             });
    //         }
    //     }

    //     if move_counter % moves_len == 0 {
    //         println!("Move: {}, Rocks: {}", move_counter, loop_counter + 1);
    //     }
    //     for (dy, row) in shape.rocks.iter().enumerate() {
    //         for (x, &is_rock) in row.iter().enumerate() {
    //             if is_rock {
    //                 world[shape.height as usize - dy][x] = true;
    //             }
    //         }
    //     }

    //     world.retain(|row| row.iter().any(|&b| b));

    //     if loop_counter == 999_999_999_999 {
    //         break;
    //     }
    //     loop_counter += 1;
    // }

    // Some(world.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
