#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Num {
    value: i64,
    order: usize,
}

impl Num {
    fn new(value: i64, order: usize) -> Self {
        Self { value, order }
    }
}

trait Nums {
    fn shift(&mut self, index: usize);
    fn to_string(&self) -> String;
}

impl Nums for Vec<Num> {
    fn shift(&mut self, order: usize) {
        let index = self.iter().position(|num| num.order == order).unwrap() as i64;
        let num = self.remove(index as usize);
        if num.value == 0 {
            self.insert(index as usize, num);
        } else {
            let len = self.len() as i64;
            let new_index = (num.value + index).rem_euclid(len);
            self.insert(new_index as usize, num);
        }
    }

    fn to_string(&self) -> String {
        self.iter()
            .map(|num| num.value.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut nums = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let int = line.parse::<i64>().unwrap();
            Num::new(int, i)
        })
        .collect::<Vec<_>>();
    let len = nums.len();

    for i in 0..len {
        let index = i % len;
        nums.shift(index);
    }

    let zero_index = nums.iter().position(|num| num.value == 0).unwrap();
    let thousand_index = (zero_index + 1000) % len;
    let two_thousand_index = (zero_index + 2000) % len;
    let three_thousand_index = (zero_index + 3000) % len;

    let thousand = nums[thousand_index].value;
    let two_thousand = nums[two_thousand_index].value;
    let three_thousand = nums[three_thousand_index].value;

    Some(thousand + two_thousand + three_thousand)
}

const DECRYPTION_KEY: i64 = 811589153;

pub fn part_two(input: &str) -> Option<i64> {
    let mut nums = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let int = line.parse::<i64>().unwrap() * DECRYPTION_KEY;
            Num::new(int, i)
        })
        .collect::<Vec<_>>();
    let len = nums.len();

    for i in 0..len * 10 {
        let index = i % len;
        nums.shift(index);
    }

    let zero_index = nums.iter().position(|num| num.value == 0).unwrap();
    let thousand_index = (zero_index + 1000) % len;
    let two_thousand_index = (zero_index + 2000) % len;
    let three_thousand_index = (zero_index + 3000) % len;

    let thousand = nums[thousand_index].value;
    let two_thousand = nums[two_thousand_index].value;
    let three_thousand = nums[three_thousand_index].value;

    Some(thousand + two_thousand + three_thousand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }

    #[test]
    fn test_shift() {
        let mut nums = vec![
            Num::new(1, 0),
            Num::new(2, 1),
            Num::new(-3, 2),
            Num::new(3, 3),
            Num::new(-2, 4),
            Num::new(0, 5),
            Num::new(4, 6),
        ];

        nums.shift(0);

        let expected = vec![
            Num::new(2, 1),
            Num::new(1, 0),
            Num::new(-3, 2),
            Num::new(3, 3),
            Num::new(-2, 4),
            Num::new(0, 5),
            Num::new(4, 6),
        ];

        assert_eq!(nums, expected);
    }
}
