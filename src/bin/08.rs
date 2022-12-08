struct Grid {
    size: usize,
    data: Vec<u32>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![0; size * size],
        }
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.data[y * self.size + x]
    }

    fn set(&mut self, x: usize, y: usize, value: u32) {
        self.data[y * self.size + x] = value;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.lines();
    let len = line.clone().count();
    let mut grid = Grid::new(len);

    for (y, line) in line.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c.to_digit(10).unwrap());
        }
    }

    let mut visibles = 0;

    for y in 0..len {
        for x in 0..len {
            if y == 0 || y == len - 1 || x == 0 || x == len - 1 {
                visibles += 1;
            } else {
                let value = grid.get(x, y);
                let mut visible_top = true;
                let mut visible_bottom = true;
                let mut visible_left = true;
                let mut visible_right = true;

                for dx in 0..x {
                    if grid.get(dx, y) >= value {
                        visible_left = false;
                        break;
                    }
                }

                for dx in x + 1..len {
                    if grid.get(dx, y) >= value {
                        visible_right = false;
                        break;
                    }
                }

                for dy in 0..y {
                    if grid.get(x, dy) >= value {
                        visible_top = false;
                        break;
                    }
                }

                for dy in y + 1..len {
                    if grid.get(x, dy) >= value {
                        visible_bottom = false;
                        break;
                    }
                }

                if visible_top || visible_bottom || visible_left || visible_right {
                    visibles += 1;
                }
            }
        }
    }

    Some(visibles)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line = input.lines();
    let len = line.clone().count();
    let mut grid = Grid::new(len);

    for (y, line) in line.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c.to_digit(10).unwrap());
        }
    }

    let mut max_distance = 0;

    for y in 0..len {
        for x in 0..len {
            if y == 0 || y == len - 1 || x == 0 || x == len - 1 {
                continue;
            }
            let value = grid.get(x, y);
            let mut distance_top = 0;
            let mut distance_bottom = 0;
            let mut distance_left = 0;
            let mut distance_right = 0;

            for dx in (0..x).rev() {
                distance_left += 1;
                if grid.get(dx, y) >= value {
                    break;
                }
            }

            for dx in x + 1..len {
                distance_right += 1;
                if grid.get(dx, y) >= value {
                    break;
                }
            }

            for dy in (0..y).rev() {
                distance_top += 1;
                if grid.get(x, dy) >= value {
                    break;
                }
            }

            for dy in y + 1..len {
                distance_bottom += 1;
                if grid.get(x, dy) >= value {
                    break;
                }
            }

            let distance = distance_top * distance_bottom * distance_left * distance_right;

            max_distance = max_distance.max(distance)
        }
    }

    Some(max_distance)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
