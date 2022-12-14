use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fmt::{Display, Error, Formatter},
};

use ndarray::Array2;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Point {
    x: usize,
    y: usize,
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
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            letter: ' ',
            height: 0,
        }
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
    points: Array2<Point>,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            points: Array2::default((w, h)),
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
                if !visited.contains(&neighbor) && neighbor.can_climb(last) {
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
        match (start.x.cmp(&0), start.y.cmp(&0)) {
            (Ordering::Greater, Ordering::Greater) => {
                neighbors.push(self.points.get((start.x - 1, start.y)).unwrap().clone());
                neighbors.push(self.points.get((start.x, start.y - 1)).unwrap().clone());
            }
            (Ordering::Greater, Ordering::Equal) => {
                neighbors.push(self.points.get((start.x - 1, start.y)).unwrap().clone());
            }
            (Ordering::Equal, Ordering::Greater) => {
                neighbors.push(self.points.get((start.x, start.y - 1)).unwrap().clone());
            }
            _ => {}
        }
        match (
            start.x.cmp(&(self.points.shape()[0] - 1)),
            start.y.cmp(&(self.points.shape()[1] - 1)),
        ) {
            (Ordering::Less, Ordering::Less) => {
                neighbors.push(self.points.get((start.x + 1, start.y)).unwrap().clone());
                neighbors.push(self.points.get((start.x, start.y + 1)).unwrap().clone());
            }
            (Ordering::Less, Ordering::Equal) => {
                neighbors.push(self.points.get((start.x + 1, start.y)).unwrap().clone());
            }
            (Ordering::Equal, Ordering::Less) => {
                neighbors.push(self.points.get((start.x, start.y + 1)).unwrap().clone());
            }
            _ => {}
        }

        neighbors
    }

    fn parse(input: &str) -> Self {
        let mut char_height = ('a'..='z')
            .enumerate()
            .map(|(i, c)| (c, i as i32 + 1))
            .collect::<HashMap<char, i32>>();

        char_height.insert('S', *char_height.get(&'a').unwrap());
        char_height.insert('E', *char_height.get(&'z').unwrap());

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut grid = Grid::new(width, height);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(height) = char_height.get(&c) {
                    grid.points[[x, y]] = Point {
                        x,
                        y,
                        letter: c,
                        height: *height,
                    };
                }
            }
        }

        grid
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);

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
    let grid = Grid::parse(input);

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
