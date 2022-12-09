use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn right(&mut self) {
        self.x += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn is_far(&self, other: &Point) -> bool {
        (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1
    }

    fn follow(&mut self, leader: &Point) {
        if self.is_far(leader) {
            if self.y + 2 <= leader.y {
                match self.x.cmp(&leader.x) {
                    std::cmp::Ordering::Greater => {
                        self.up();
                        self.left();
                    }
                    std::cmp::Ordering::Equal => {
                        self.up();
                    }
                    std::cmp::Ordering::Less => {
                        self.up();
                        self.right();
                    }
                }
            } else if self.y - 2 >= leader.y {
                match self.x.cmp(&leader.x) {
                    std::cmp::Ordering::Greater => {
                        self.down();
                        self.left();
                    }
                    std::cmp::Ordering::Equal => {
                        self.down();
                    }
                    std::cmp::Ordering::Less => {
                        self.down();
                        self.right();
                    }
                }
            } else if self.x + 2 <= leader.x {
                match self.y.cmp(&leader.y) {
                    std::cmp::Ordering::Greater => {
                        self.right();
                        self.down();
                    }
                    std::cmp::Ordering::Equal => {
                        self.right();
                    }
                    std::cmp::Ordering::Less => {
                        self.right();
                        self.up();
                    }
                }
            } else if self.x - 2 >= leader.x {
                match self.y.cmp(&leader.y) {
                    std::cmp::Ordering::Greater => {
                        self.left();
                        self.down();
                    }
                    std::cmp::Ordering::Equal => {
                        self.left();
                    }
                    std::cmp::Ordering::Less => {
                        self.left();
                        self.up();
                    }
                }
            } else {
                panic!("{}, {} -> {}, {}", self.x, self.y, leader.x, leader.y);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut tail_visited = HashSet::new();
    tail_visited.insert(tail.clone());

    for line in lines {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().chars().next().unwrap();
        let distance = split.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..distance {
            match direction {
                'R' => head.right(),
                'L' => head.left(),
                'U' => head.up(),
                'D' => head.down(),
                _ => panic!("Unknown direction"),
            }

            tail.follow(&head);
            tail_visited.insert(tail.clone());
        }
    }

    Some(tail_visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut points: Vec<Point> = vec![
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
    ];

    let mut tail_visited = HashSet::new();
    tail_visited.insert(Point { x: 0, y: 0 });

    for line in lines {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().chars().next().unwrap();
        let distance = split.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..distance {
            let head = &mut points[0];

            match direction {
                'R' => head.right(),
                'L' => head.left(),
                'U' => head.up(),
                'D' => head.down(),
                _ => panic!("Unknown direction"),
            }

            for i in 0..9 {
                let leader = points[i].clone();
                let follower = &mut points[i + 1];

                follower.follow(&leader);
            }
            tail_visited.insert(points[9].clone());
        }
    }

    Some(tail_visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
