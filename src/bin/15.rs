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

// 340ms

pub fn part_two(input: &str) -> Option<u64> {
    let max_x = 4_000_000;

    let sensors_with_dists: Vec<(Sensor, u32)> = input
        .lines()
        .map(|line| {
            let pair = parse_line(line).unwrap().1;

            let dist = distance(&pair.sensor, &pair.beacon);

            (pair.sensor, dist)
        })
        .collect::<Vec<_>>();

    let mut y = 0;
    let mut x = 0;
    let mut can_exit = false;
    'outer: loop {
        let point = Beacon::new(x, y);

        for (sensor, dist) in &sensors_with_dists {
            let dist_to_s = distance(&point, sensor);

            if dist_to_s > *dist {
                can_exit = true;
            } else {
                can_exit = false;
                // not distress

                // jump to next location
                let x_to = *dist as i32 - (y - sensor.y).abs() + sensor.x + 1;

                if x_to > max_x {
                    x = 0;
                    y += 1;
                } else {
                    x = x_to;
                }

                continue 'outer;
            }
        }

        if can_exit {
            break;
        }
    }

    let frequency = (x as u64) * 4_000_000 + y as u64;

    Some(frequency)
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
        assert_eq!(part_two(&input), Some(56000011));
    }
}
