use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn parse(input: &str) -> Self {
        let mut split = input.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();

        Self { x, y, z }
    }

    /// Find 6 neighbors of a cube
    fn neighbors(&self) -> Vec<Self> {
        let neighbors = vec![
            // Up
            Self {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            // Down
            Self {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            // Left
            Self {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            // Right
            Self {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            // Front
            Self {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            // Back
            Self {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
        ];

        neighbors
    }

    /// Finds all cubes while moving from `self` to `target` using dfs
    /// If `cubes` contains `self` or `target`, they will be skipped
    fn find_cubes(&self, target: &Self, cubes: &HashSet<Self>, min: i32, max: i32) -> Vec<Self> {
        let mut cubes = cubes.clone();
        cubes.remove(self);
        cubes.remove(target);

        let mut stack = vec![self.clone()];
        let mut visited = HashSet::new();
        let mut outsides = vec![];

        while let Some(cube) = stack.pop() {
            if visited.contains(&cube) {
                continue;
            }

            visited.insert(cube.clone());

            let is_outside = cube.x < min
                || cube.x > max
                || cube.y < min
                || cube.y > max
                || cube.z < min
                || cube.z > max;

            if !is_outside {
                outsides.push(cube.clone());
            }

            for neighbor in cube.neighbors() {
                if !visited.contains(&neighbor) && !cubes.contains(&neighbor) && !is_outside {
                    stack.push(neighbor);
                }
            }
        }

        outsides
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let cubes: HashSet<Cube> = input.lines().map(Cube::parse).collect();

    let cubes_sides = count_sides(&cubes);

    Some(cubes_sides)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: HashSet<Cube> = (0..25)
        .flat_map(|x| (0..25).flat_map(move |y| (0..25).map(move |z| Cube { x, y, z })))
        .collect();

    let cubes: HashSet<Cube> = input.lines().map(Cube::parse).collect();

    let outsides = Cube {
        x: -1,
        y: -1,
        z: -1,
    }
    .find_cubes(
        &Cube {
            x: 24,
            y: 24,
            z: 24,
        },
        &cubes,
        -1,
        24,
    );

    let cubes_plus_outsides: HashSet<Cube> = cubes
        .union(&outsides.iter().cloned().collect())
        .cloned()
        .collect();

    let insides: HashSet<Cube> = grid.difference(&cubes_plus_outsides).cloned().collect();

    let cubes_sides = count_sides(&cubes);
    let insides_sides = count_sides(&insides);

    Some(cubes_sides - insides_sides)
}

fn count_sides(cubes: &HashSet<Cube>) -> u32 {
    let len = cubes.len();
    let mut duplicates_count = 0;

    for cube in cubes {
        for new_cube in cube.neighbors() {
            if cubes.contains(&new_cube) {
                duplicates_count += 1;
            }
        }
    }

    (len * 6 - duplicates_count) as u32
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
