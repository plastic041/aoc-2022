use advent_of_code::helpers::Directory;

pub fn part_one(input: &str) -> Option<i32> {
    let mut dir = Directory::new("/".to_string(), 0);
    let mut paths: Vec<String> = vec![];

    let mut lines = input.lines();

    lines.next();

    for line in lines {
        if line.starts_with("$ cd") {
            // command
            let mut parts = line.split_whitespace();
            let _ = parts.next(); // $
            let _ = parts.next(); // cd
            let dir_name = parts.next().unwrap();

            match dir_name {
                ".." => {
                    paths.pop();
                }
                name => {
                    paths.push(name.to_string());
                }
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            // "size file_name"
            let mut parts = line.split_whitespace();
            let size = parts.next().unwrap().parse::<i32>().unwrap();
            let name = parts.next().unwrap();

            let mut current = &mut dir;
            for path in &paths {
                current = current
                    .children
                    .entry(path.to_string())
                    .or_insert_with(|| Directory::new(path.to_string(), 0));
            }

            current.add_child(name.to_string(), Directory::new(name.to_string(), size));
        }
    }

    let dirs = dir.get_directories();

    let max = 100000;

    let mut sum = 0;

    for d in dirs {
        if d.get_size() <= max {
            sum += d.get_size();
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dir = Directory::new("/".to_string(), 0);
    let mut paths: Vec<String> = vec![];

    let mut lines = input.lines();

    lines.next();

    for line in lines {
        if line.starts_with("$ cd") {
            // command
            let mut parts = line.split_whitespace();
            let _ = parts.next(); // $
            let _ = parts.next(); // cd
            let dir_name = parts.next().unwrap();

            match dir_name {
                ".." => {
                    paths.pop();
                }
                name => {
                    paths.push(name.to_string());
                }
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            // "size file_name"
            let mut parts = line.split_whitespace();
            let size = parts.next().unwrap().parse::<i32>().unwrap();
            let name = parts.next().unwrap();

            let mut current = &mut dir;
            for path in &paths {
                current = current
                    .children
                    .entry(path.to_string())
                    .or_insert_with(|| Directory::new(path.to_string(), 0));
            }

            current.add_child(name.to_string(), Directory::new(name.to_string(), size));
        }
    }

    let mut dirs = dir.get_directories();
    let device_size = 70000000;
    let device_size_remain = device_size - dir.get_size();
    let should_be = 30000000;

    dirs.sort_unstable_by_key(|d| d.get_size());

    let first = dirs
        .into_iter()
        .find(|d| {
            let remain = device_size_remain + d.get_size();
            remain >= should_be
        })
        .unwrap();

    Some(first.get_size())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
