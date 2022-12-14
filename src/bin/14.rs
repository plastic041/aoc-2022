use std::{cmp::Ordering, fmt::Display};

use ndarray::{s, Array2};

#[derive(Default, Debug)]
enum BlockState {
    #[default]
    Empty,
    Rock,
    Sand,
}

impl Display for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockState::Empty => write!(f, "."),
            BlockState::Rock => write!(f, "#"),
            BlockState::Sand => write!(f, "o"),
        }
    }
}

#[derive(Default, Debug)]
struct Block {
    state: BlockState,
}

impl Block {
    fn is_blocked(&self) -> bool {
        match self.state {
            BlockState::Empty => false,
            BlockState::Rock => true,
            BlockState::Sand => true,
        }
    }
}

struct Grid(Array2<Block>);

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid(Array2::default((width, height)))
    }

    fn fall(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let current = &self.0[[x, y]];
        if current.is_blocked() {
            panic!("Block ({}, {}) is blocked", x, y);
        }

        let belows = &self.0.slice(s![x, y + 1..]);
        for (relative_y, below) in belows.indexed_iter() {
            let below_y = y + relative_y + 1;

            if below.is_blocked() {
                if let Some(left_down) = self.0.get((x - 1, below_y)) {
                    if !left_down.is_blocked() {
                        return self.fall(x - 1, below_y);
                    }
                }

                if let Some(right_down) = self.0.get((x + 1, below_y)) {
                    if !right_down.is_blocked() {
                        return self.fall(x + 1, below_y);
                    }
                }

                return Some((x, below_y - 1));
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    for line in input.lines() {
        let mut path = Vec::new();
        let split = line.split(" -> ");

        for xy in split {
            let mut coord: (usize, usize) = (0, 0);
            let mut split = xy.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();

            coord.0 = x;
            coord.1 = y;

            path.push(coord);
        }

        paths.push(path);
    }

    let max_x = *paths
        .iter()
        .map(|path| path.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();
    let max_y = *paths
        .iter()
        .map(|path| path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();

    let mut grid: Grid = Grid::new(max_x + 1, max_y + 1);

    for path in paths {
        let (x, y) = path[0];
        grid.0[[x, y]] = Block {
            state: BlockState::Rock,
        };
        for i in 0..path.len() {
            let current = path.get(i).unwrap();
            let next = path.get(i + 1);

            if let Some(next) = next {
                let mut x = current.0;
                let mut y = current.1;

                while x != next.0 || y != next.1 {
                    // While we're not at the next coord, move towards it
                    match x.cmp(&next.0) {
                        Ordering::Less => x += 1,
                        Ordering::Greater => x -= 1,
                        Ordering::Equal => (),
                    }

                    match y.cmp(&next.1) {
                        Ordering::Less => y += 1,
                        Ordering::Greater => y -= 1,
                        Ordering::Equal => (),
                    }

                    grid.0[[x, y]] = Block {
                        state: BlockState::Rock,
                    };
                }
            }
        }
    }

    // // print grid
    // // 494 <= x <= 503
    // // 0 <= y <= 9
    // for y in 0..=9 {
    //     for x in 494..=503 {
    //         let block = &grid.0[[x, y]];
    //         match block.state {
    //             BlockState::Empty => print!("."),
    //             BlockState::Rock => print!("#"),
    //             BlockState::Sand => print!("o"),
    //         }
    //     }
    //     println!();
    // }

    let start = (500, 0);

    let mut count = 0;

    while let Some((x, y)) = grid.fall(start.0, start.1) {
        count += 1;
        grid.0[[x, y]] = Block {
            state: BlockState::Sand,
        };
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

    for line in input.lines() {
        let mut path = Vec::new();
        let split = line.split(" -> ");

        for xy in split {
            let mut coord: (usize, usize) = (0, 0);
            let mut split = xy.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();

            coord.0 = x;
            coord.1 = y;

            path.push(coord);
        }

        paths.push(path);
    }

    let max_x = 5000;
    let max_y = *paths
        .iter()
        .map(|path| path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();

    let mut grid: Grid = Grid::new(max_x, max_y + 3);

    // fill max_y + 2 with rocks
    for x in 0..max_x {
        grid.0[[x, max_y + 2]] = Block {
            state: BlockState::Rock,
        };
    }

    for path in paths {
        let (x, y) = path[0];
        grid.0[[x, y]] = Block {
            state: BlockState::Rock,
        };
        for i in 0..path.len() {
            let current = path.get(i).unwrap();
            let next = path.get(i + 1);

            if let Some(next) = next {
                let mut x = current.0;
                let mut y = current.1;

                while x != next.0 || y != next.1 {
                    // While we're not at the next coord, move towards it
                    match x.cmp(&next.0) {
                        Ordering::Less => x += 1,
                        Ordering::Greater => x -= 1,
                        Ordering::Equal => (),
                    }

                    match y.cmp(&next.1) {
                        Ordering::Less => y += 1,
                        Ordering::Greater => y -= 1,
                        Ordering::Equal => (),
                    }

                    grid.0[[x, y]] = Block {
                        state: BlockState::Rock,
                    };
                }
            }
        }
    }

    let start = (500, 0);

    let mut count = 0;

    while let Some((x, y)) = grid.fall(start.0, start.1) {
        count += 1;
        grid.0[[x, y]] = Block {
            state: BlockState::Sand,
        };
        if (x, y) == (500, 0) {
            break;
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
