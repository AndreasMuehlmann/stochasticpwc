use std::fs::File;
use std::io::{self, BufReader, BufRead};

use crate::pattern_trees::Follower;
use crate::pattern_trees::PatternTree;
use crate::pattern_trees::PatternTrees;


pub struct PatternTreesFactory {
    count_pattern_trees: usize,
}

impl PatternTreesFactory {
    pub fn new(count_pattern_trees: usize) -> Self {
        Self {
            count_pattern_trees,
        }
    }

    pub fn from_paths_error_handling(&self, paths: Vec<String>) -> PatternTrees {
        let pattern_trees: PatternTrees;
        if paths.len() > 1 {
            pattern_trees = self.from_password_lists(&paths[1..]).unwrap_or_else(|err| {
                eprint!("{}", err);
                Self::from_encoding_error_handling()
            });
        }
        else {
            pattern_trees = Self::from_encoding_error_handling();
        }
        pattern_trees
    }

    pub fn from_encoding_error_handling() -> PatternTrees {
        let mut path = "pattern_tree_encoding.txt".to_string();
        let pattern_trees = loop {
            match Self::from_encoding(&path) {
                Ok(pattern_trees) => break pattern_trees,
                Err(err) => {
                    eprintln!("Error opening file: {}", err);
                    println!("input a valid file path for a pattern tree encoding");
                    io::stdin()
                        .read_line(&mut path)
                        .expect("Failed to read from stdin");
                }
            }
        };
        pattern_trees
    }

    pub fn from_password_lists(&self, paths: &[String]) -> Result<PatternTrees, io::Error> {
        let mut pattern_trees: Vec<PatternTree> = vec![];
        for _ in 0..self.count_pattern_trees {
            pattern_trees.push(PatternTree::new());
        }
        for path in paths {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = match line {
                    Ok(line_content) => line_content,
                    Err(err) => {
                        eprintln!("Error reading line in pattern_trees_from_pw_lists: {}", err);
                        continue;
                    }
                };

                let line = line.trim();
                if line.is_empty() {
                    continue
                }

                let mut tail: String = line.to_string();
                for _ in line.chars() {
                    for pattern_length in 0..tail.len().min(self.count_pattern_trees) {
                        let (pattern, following_letter) = Self::split_kv_pair(&tail, pattern_length);
                        Self::insert_kv_pair(&mut pattern_trees[pattern_length], pattern, Follower::new(1, following_letter));
                    }
                    tail = tail[1..].chars().collect();
                }
            }
        }
        for pattern_tree in pattern_trees.iter_mut() {
            for followers in pattern_tree.values_mut() {
                followers.retain(|follower| follower.count != 1);
                followers.sort_by(|a, b| b.count.cmp(&a.count));
            }
        }
        Ok(PatternTrees::new(pattern_trees))
    }

    pub fn from_encoding(path: &str) -> Result<PatternTrees, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut pattern_length: usize = 0;
        let mut pattern_tree: PatternTree = PatternTree::new();
        let mut pattern_trees: Vec<PatternTree> = vec![];

        for line in reader.lines() {
            let line = match line {
                Ok(line_content) => line_content,
                Err(err) => {
                    eprintln!("Error reading line in parse_pattern_trees: {}", err);
                    continue;
                }
            };

            let line = line.trim();
            if line.is_empty() {
                continue
            }
            if line == "---" {
                pattern_trees.push(pattern_tree);
                pattern_tree = PatternTree::new();
                pattern_length += 1;
                continue;
            }

            let (pattern, following_letter) = Self::split_kv_pair(line, pattern_length);
            let count = Self::count_from_encoding(line, pattern_length);
            Self::insert_kv_pair(&mut pattern_tree, pattern, Follower::new(count, following_letter))
        }
        Ok(PatternTrees::new(pattern_trees))
    }

    fn split_kv_pair(text: &str, mut pattern_length: usize) -> (String, char) {
        let mut pattern = "".to_string();
        let mut following_letter: char = '\0';
        for letter in text.chars() {
            if pattern_length <= 0 {
                following_letter = letter;
                break; 
            }
            pattern.push(letter);
            pattern_length -= 1;
        }
        (pattern, following_letter)
    }

    fn count_from_encoding(line: &str, pattern_length: usize) -> u32 {
        let mut count = "".to_string();
        for (index, letter) in line.chars().enumerate() {
            if index > pattern_length {
                count.push(letter);
            }
        }
        match count.parse::<u32>() {
            Ok(count) => count,
            Err(err) => {
                eprintln!("Error parsing count: {}", err);
                0
            },
        }
    }

    fn insert_kv_pair(pattern_tree: &mut PatternTree, pattern: String, new_follower: Follower) {
        if let Some(followers) = pattern_tree.get_mut(&pattern) {
            for follower in followers.iter_mut() {
                if follower.letter == new_follower.letter {
                    follower.count += 1; 
                    return;
                }
            }
            followers.push(new_follower);
        } else {
            let mut followers = Vec::new();
            followers.push(new_follower);
            pattern_tree.insert(pattern, followers);
        }
    }

}