#[derive(Debug)]
enum Command {
    Step(usize),
    Turn(DirectionCommand),
}

#[derive(Debug)]
enum DirectionCommand {
    Left,
    Right,
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mut current_number = 0;
    let mut commands = Vec::new();

    for (i, c) in input.chars().enumerate() {
        match c {
            '0'..='9' => {
                current_number *= 10;
                current_number += c.to_digit(10).unwrap() as usize;

                if i == input.len() - 1 {
                    commands.push(Command::Step(current_number));
                }
            }
            'L' => {
                commands.push(Command::Step(current_number));
                commands.push(Command::Turn(DirectionCommand::Left));
                current_number = 0;
            }
            'R' => {
                commands.push(Command::Step(current_number));
                commands.push(Command::Turn(DirectionCommand::Right));
                current_number = 0;
            }
            _ => panic!("unexpected character"),
        }
    }

    commands
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Wall,
    Open,
    Void,
}

impl Map {
    fn width(&self) -> usize {
        // longest row width
        self.cells.iter().map(|row| row.len()).max().unwrap()
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn get_next_point(&self, point: &Point, direction: &Direction) -> Point {
        let x = point.x;
        let y = point.y;

        let width = self.width();
        let height = self.height();

        match direction {
            Direction::Up => {
                if y == 0 {
                    Point::new(x, height - 1)
                } else {
                    Point::new(x, y - 1)
                }
            }
            Direction::Down => {
                if y == height - 1 {
                    Point::new(x, 0)
                } else {
                    Point::new(x, y + 1)
                }
            }
            Direction::Left => {
                if x == 0 {
                    Point::new(width - 1, y)
                } else {
                    Point::new(x - 1, y)
                }
            }
            Direction::Right => {
                if x == width - 1 {
                    Point::new(0, y)
                } else {
                    Point::new(x + 1, y)
                }
            }
        }
    }

    fn get_cell(&self, point: &Point) -> Cell {
        // self.cells[point.y][point.x].clone()
        self.cells
            .get(point.y)
            .and_then(|row| row.get(point.x))
            .cloned()
            .unwrap_or(Cell::Void)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut split = input.split("\n\n");
    let map = Map {
        cells: split
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Open,
                        ' ' => Cell::Void,
                        _ => panic!("unexpected character"),
                    })
                    .collect()
            })
            .collect(),
    };

    let desc = split.next().unwrap();

    let commands = parse_commands(desc);

    let start: Point = Point::new(
        map.cells[0].iter().position(|c| *c == Cell::Open).unwrap(),
        0,
    );

    let mut current_direction = Direction::Right;
    let mut current_point = start;

    for command in &commands {
        match command {
            Command::Step(step) => {
                let mut i: i32 = 0;

                println!("{:?} {:?}, {}", current_direction, current_point, step);
                while i < *step as i32 {
                    let next_point = map.get_next_point(&current_point, &current_direction);
                    let cell = map.get_cell(&next_point);

                    match cell {
                        Cell::Open => current_point = next_point,
                        Cell::Wall => break,
                        Cell::Void => {
                            let mut check = map.get_next_point(&current_point, &current_direction);
                            while map.get_cell(&check) == Cell::Void {
                                check = map.get_next_point(&check, &current_direction);
                            }
                            match map.get_cell(&check) {
                                Cell::Open => {
                                    current_point = check;
                                }
                                Cell::Wall => break,
                                Cell::Void => panic!("unexpected cell"),
                            }
                        }
                    }

                    i += 1;
                }
            }
            Command::Turn(direction) => {
                current_direction = match &direction {
                    DirectionCommand::Left => match &current_direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    },
                    DirectionCommand::Right => match &current_direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    },
                }
            }
        }
    }

    let row = current_point.y + 1;
    let column = current_point.x + 1;
    let answer = (row * 1000) + (column * 4) + current_direction as usize;
    let point = Point::new(column, row);
    println!("{:?}", map.get_cell(&point));

    println!(
        "row: {}, column: {}, direction: {:?}",
        row, column, current_direction
    );

    Some(answer)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
