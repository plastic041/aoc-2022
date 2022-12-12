use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Error, Formatter},
};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Point {
    x: i32,
    y: i32,
    letter: char,
    height: i32,
}

impl Point {
    /// Tests if a point can move(climb) to another point.
    /// ```
    /// use advent_of_code_2018::day_12::Point;
    ///
    /// let point = Point { x: 0, y: 0, height: 10 };
    /// let can_climb_higher = Point { x: 0, y: 1, height: 11 };
    /// let can_climb_same = Point { x: 0, y: 1, height: 10 };
    /// let can_climb_lower = Point { x: 0, y: 1, height: 5 };
    /// let cannot_climb = Point { x: 0, y: 1, height: 2 };
    ///
    /// assert!(point.can_climb(&can_climb_higher));
    /// assert!(point.can_climb(&can_climb_same));
    /// assert!(point.can_climb(&can_climb_lower));
    /// assert!(!point.can_climb(&cannot_climb));
    /// ```
    fn can_climb(&self, other: &Point) -> bool {
        other.height - self.height <= 1
    }

    fn can_decend(&self, other: &Point) -> bool {
        self.height - other.height <= 1
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // pad 4 spaces
        // 1 -> "  1 "
        // 10 -> " 10 "
        write!(f, "{: >4} {}x{} ", self.height, self.x, self.y)
    }
}

#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
    points: Vec<Point>,
}

impl Grid {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            points: Vec::new(),
        }
    }

    /// find shortest path from start to end using bfs
    fn shortest_path(&self, start: &Point, end: &Point) -> Option<Vec<Point>> {
        // queue of paths
        let mut queue: VecDeque<Vec<Point>> = VecDeque::new();
        // keep track of visited points
        let mut visited: Vec<Point> = Vec::new();

        // push start point
        queue.push_back(vec![start.clone()]);

        // while queue is not empty
        while let Some(path) = queue.pop_front() {
            // get last point in path
            let last = path.last().unwrap();

            // if last point is end point, return path
            if last == end {
                return Some(path);
            }

            // get neighbors of last point
            for neighbor in self.neighbors(last) {
                // if neighbor is not visited and can climb to neighbor
                if !visited.contains(&neighbor) && last.can_climb(&neighbor) {
                    // add neighbor to path and push path to queue
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back(new_path);
                    // add neighbor to visited
                    visited.push(neighbor);
                }
            }
        }

        // no path found
        None
    }

    /// finds shortest path from start to point with input height
    fn shortest_path_to_height(&self, start: &Point, height: i32) -> Option<Vec<Point>> {
        // queue of paths
        let mut queue: VecDeque<Vec<Point>> = VecDeque::new();
        // keep track of visited points
        let mut visited: Vec<Point> = Vec::new();

        // push start point
        queue.push_back(vec![start.clone()]);

        // while queue is not empty
        while let Some(path) = queue.pop_front() {
            // get last point in path
            let last = path.last().unwrap();

            // if last point is end point, return path
            if last.height == height {
                return Some(path);
            }

            // get neighbors of last point
            for neighbor in self.neighbors(last) {
                // if neighbor is not visited and can decend to neighbor
                if !visited.contains(&neighbor) && last.can_decend(&neighbor) {
                    // add neighbor to path and push path to queue
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back(new_path);
                    // add neighbor to visited
                    visited.push(neighbor);
                }
            }
        }

        // no path found
        None
    }

    fn neighbors(&self, start: &Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        for other_point in &self.points {
            if start.x == other_point.x && (start.y - other_point.y).abs() == 1 {
                // up or down
                neighbors.push(other_point.clone());
            }
            if start.y == other_point.y && (start.x - other_point.x).abs() == 1 {
                // left or right
                neighbors.push(other_point.clone());
            }
        }
        neighbors
    }

    fn get(&self, x: i32, y: i32) -> Option<&Point> {
        // find point
        let index = y * self.width + x;

        if index < self.points.len() as i32 {
            Some(&self.points[index as usize])
        } else {
            None
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = self.get(x, y).unwrap();
                write!(f, "{}", point)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut char_height = ('a'..='z')
        .enumerate()
        .map(|(i, c)| (c, i as i32 + 1))
        .collect::<HashMap<char, i32>>();

    char_height.insert('S', *char_height.get(&'a').unwrap());
    char_height.insert('E', *char_height.get(&'z').unwrap());

    let mut grid = Grid::new();
    grid.width = input.lines().next().unwrap().len() as i32;
    grid.height = input.lines().count() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(height) = char_height.get(&c) {
                grid.points.push(Point {
                    x: x as i32,
                    y: y as i32,
                    letter: c,
                    height: *height,
                });
            }
        }
    }

    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let start_point = grid
        .points
        .iter()
        .find(|p| p.letter == 'S')
        .unwrap()
        .clone();

    let end_point = grid
        .points
        .iter()
        .find(|p| p.letter == 'E')
        .unwrap()
        .clone();

    let path = grid.shortest_path(&start_point, &end_point).unwrap();

    Some(path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);

    let start_point = grid
        .points
        .iter()
        .find(|p| p.letter == 'E')
        .unwrap()
        .clone();

    let path = grid.shortest_path_to_height(&start_point, 1).unwrap();

    Some(path.len() as u32 - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
