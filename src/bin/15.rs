use nom::{bytes::complete::tag, IResult};

trait Point {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

#[derive(Debug, Clone)]
struct Sensor {
    x: i32,
    y: i32,
}

impl Sensor {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Point for Sensor {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug, Clone)]
struct Beacon {
    x: i32,
    y: i32,
}

impl Beacon {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Point for Beacon {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug)]
struct Pair {
    sensor: Sensor,
    beacon: Beacon,
}

impl Pair {
    fn new(sx: i32, sy: i32, bx: i32, by: i32) -> Self {
        Self {
            sensor: Sensor::new(sx, sy),
            beacon: Beacon::new(bx, by),
        }
    }
}

fn distance<T, U>(a: &T, b: &U) -> u32
where
    T: Point,
    U: Point,
{
    let x = (a.x() - b.x()).unsigned_abs();
    let y = (a.y() - b.y()).unsigned_abs();
    x + y
}

/// Parse a line of input.
/// Returns the sensor and beacon.
/// `Sensor at x=[i32], y=[i32]: closest beacon is at x=[i32], y=[i32]`
fn parse_line(line: &str) -> IResult<&str, Pair> {
    let (input, _) = tag("Sensor at x=")(line)?;
    let (input, x) = nom::character::complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = nom::character::complete::i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, bx) = nom::character::complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, by) = nom::character::complete::i32(input)?;

    Ok((input, Pair::new(x, y, bx, by)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let check_y = 2000000;

    let pairs: Vec<Pair> = input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let mut checked = 0;

    for pair in &pairs {
        let dist = distance(&pair.sensor, &pair.beacon) as i32;
        if pair.sensor.y - dist <= check_y && check_y <= pair.sensor.y + dist {
            let dist = dist - (check_y - pair.sensor.y).abs();
            let from = pair.sensor.x - dist;
            let to = pair.sensor.x + dist;

            for _ in from..=to {
                checked += 1;
            }
        }
    }

    pairs.iter().for_each(|pair| {
        let beacon = &pair.beacon;

        if beacon.y == check_y {
            checked -= 1;
        }
    });

    Some(checked)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
