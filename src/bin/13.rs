use std::cmp::Ordering;

#[derive(Debug)]
enum OrderResult {
    Right,
    NotRight,
    Undetermined,
}

#[derive(Debug, PartialEq, Eq)]
enum Data {
    I32(i32),
    Vec(Vec<Data>),
}

impl Data {
    fn parse(input: &str) -> Self {
        let mut chars = input.chars();
        let first = chars.next();

        if let Some(first) = first {
            if first == '[' {
                let mut vec = Vec::new();
                let mut current = String::new();
                let mut depth = 1;
                for c in chars {
                    match c {
                        '[' => {
                            depth += 1;
                            current.push(c);
                        }
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                vec.push(Data::parse(&current));
                                current = String::new();
                            } else {
                                current.push(c);
                            }
                        }
                        ',' => {
                            if depth == 1 {
                                vec.push(Data::parse(&current));
                                current = String::new();
                            } else {
                                current.push(c);
                            }
                        }
                        _ => current.push(c),
                    }
                }
                Data::Vec(vec)
            } else {
                Data::I32(input.parse().unwrap())
            }
        } else {
            // Empty string, []
            Data::Vec(Vec::new())
        }
    }

    fn compare(&self, other: &Data) -> OrderResult {
        match (self, other) {
            (Data::I32(left), Data::I32(right)) => compare_int(left, right),
            (Data::Vec(left), Data::Vec(right)) => compare_vec(left, right),
            (Data::I32(left), Data::Vec(right)) => compare_int_vec(left, right),
            (Data::Vec(left), Data::I32(right)) => compare_vec_int(left, right),
        }
    }
}

fn compare_int_vec(left: &i32, right: &[Data]) -> OrderResult {
    let left_vec = vec![Data::I32(*left)];
    compare_vec(&left_vec, right)
}

fn compare_vec_int(left: &[Data], right: &i32) -> OrderResult {
    let right_vec = vec![Data::I32(*right)];
    compare_vec(left, &right_vec)
}

fn compare_int(left: &i32, right: &i32) -> OrderResult {
    match left.cmp(right) {
        Ordering::Less => OrderResult::Right,
        Ordering::Equal => OrderResult::Undetermined,
        Ordering::Greater => OrderResult::NotRight,
    }
}

fn compare_vec(left: &[Data], right: &[Data]) -> OrderResult {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    loop {
        match (left_iter.next(), right_iter.next()) {
            (Some(left), Some(right)) => match left.compare(right) {
                OrderResult::Right => return OrderResult::Right,
                OrderResult::NotRight => return OrderResult::NotRight,
                OrderResult::Undetermined => continue,
            },
            (Some(_), None) => return OrderResult::NotRight,
            (None, Some(_)) => return OrderResult::Right,
            (None, None) => return OrderResult::Undetermined,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    data: Data,
}

impl Packet {
    fn parse(input: &str) -> Self {
        let data = Data::parse(input);
        Packet { data }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        let order_result = self.data.compare(&other.data);
        match order_result {
            OrderResult::Right => Some(Ordering::Less),
            OrderResult::NotRight => Some(Ordering::Greater),
            OrderResult::Undetermined => Some(Ordering::Equal),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input.split("\n\n");
    let mut right_value: u32 = 0;

    for (i, pair) in pairs.enumerate() {
        let pair_number = i as u32 + 1;
        let mut lines = pair.lines();

        let left = Packet::parse(lines.next().unwrap());
        let right = Packet::parse(lines.next().unwrap());

        match left.data.compare(&right.data) {
            OrderResult::Right => right_value += pair_number,
            OrderResult::NotRight => (),
            OrderResult::Undetermined => (),
        }
    }

    Some(right_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Packet::parse(line))
            }
        })
        .collect::<Vec<_>>();

    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));

    packets.sort();

    let index_of_2 = packets
        .iter()
        .position(|p| p.data == Data::parse("[[2]]"))
        .unwrap() as u32
        + 1;
    let index_of_6 = packets
        .iter()
        .position(|p| p.data == Data::parse("[[6]]"))
        .unwrap() as u32
        + 1;

    Some(index_of_2 * index_of_6)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    /// Test that the Data parser works
    fn test_data_parse() {
        let input = "[1,1,3,1,1]";
        let data = Data::parse(input);
        assert_eq!(
            data,
            Data::Vec(vec![
                Data::I32(1),
                Data::I32(1),
                Data::I32(3),
                Data::I32(1),
                Data::I32(1)
            ])
        );

        let input = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let data = Data::parse(input);
        assert_eq!(
            data,
            Data::Vec(vec![
                Data::I32(1),
                Data::Vec(vec![
                    Data::I32(2),
                    Data::Vec(vec![
                        Data::I32(3),
                        Data::Vec(vec![
                            Data::I32(4),
                            Data::Vec(vec![Data::I32(5), Data::I32(6), Data::I32(0)])
                        ])
                    ])
                ]),
                Data::I32(8),
                Data::I32(9)
            ])
        );
    }
}
