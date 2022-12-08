use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};

pub fn alphabet_score(character: char) -> i32 {
    match character {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid character"),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Directory {
    pub name: String,
    pub size: i32,
    pub children: BTreeMap<String, Directory>,
}

impl Directory {
    pub fn new(name: String, size: i32) -> Directory {
        Directory {
            name,
            size,
            children: BTreeMap::new(),
        }
    }

    pub fn add_child(&mut self, file_name: String, child: Directory) {
        self.children.insert(file_name, child);
    }

    pub fn get_size(&self) -> i32 {
        let mut size = self.size;
        for child in &self.children {
            size += child.1.get_size();
        }
        size
    }

    pub fn get_child(&self, names: Vec<&String>) -> Option<&Directory> {
        let mut current = self;

        for name in names {
            match current.children.get(name) {
                Some(child) => {
                    current = child;
                }
                None => {
                    return None;
                }
            }
        }

        Some(current)
    }

    pub fn get_directories(&self) -> Vec<&Directory> {
        let mut directories = vec![];

        for child in &self.children {
            if child.1.children.is_empty() {
                // file
                continue;
            } else {
                // recursive
                directories.push(child.1);

                let mut child_directories = child.1.get_directories();
                directories.append(&mut child_directories);
            }
        }

        directories
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.children.is_empty() {
            // file
            write!(f, "{} (file, size={})", self.name, self.size)
        } else {
            // directory
            write!(f, "{} (dir, size={})", self.name, self.size)
        }
    }
}
