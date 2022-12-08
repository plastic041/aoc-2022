use std::collections::BTreeMap;

#[derive(Debug)]
struct Entry {
    name: String,
    is_file: bool,
    size: i32,
    children: BTreeMap<String, Entry>,
}

impl Entry {
    fn new(name: String, size: i32) -> Self {
        Entry {
            name,
            is_file: size > 0,
            size,
            children: BTreeMap::new(),
        }
    }

    fn add_entry(&mut self, child: Entry) {
        self.children.insert(child.name.to_string(), child);
    }

    fn get_dirs(&self) -> Vec<&Entry> {
        let mut children = vec![self];

        for child in &self.children {
            if !child.1.is_file {
                children.append(&mut child.1.get_dirs());
            }
        }

        children
    }

    fn get_size(&self) -> i32 {
        match self.is_file {
            true => self.size,
            false => self.children.iter().map(|d| d.1.get_size()).sum(),
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut dir = Entry::new("/".to_string(), 0);
    let mut paths: Vec<String> = vec![];

    let mut lines = input.lines();

    lines.next(); // skips cd /

    for line in lines {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else if line.starts_with("$ cd") {
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
                    .or_insert_with(|| Entry::new(name.to_string(), 0));
            }

            current.add_entry(Entry::new(name.to_string(), size));
        }
    }

    let dirs = dir.get_dirs();

    println!("dirs: {:#?}", dirs);
    let sizes = dirs.iter().map(|d| d.get_size()).collect::<Vec<i32>>();
    println!("sizes: {:?}", sizes);
    let max = 100000;
    let sizes_below_max: i32 = sizes.iter().filter(|s| **s < max).sum();

    Some(sizes_below_max)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dir = Entry::new("/".to_string(), 0);
    let mut paths: Vec<String> = vec![];

    let mut lines = input.lines();

    lines.next();

    for line in lines {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else if line.starts_with("$ cd") {
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
                    .or_insert_with(|| Entry::new(name.to_string(), 0));
            }

            current.add_entry(Entry::new(name.to_string(), size));
        }
    }

    let mut dirs = dir.get_dirs();
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
