use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Write};


pub struct Follower {
    pub count: u32,
    pub letter: char,
}

impl Follower {
    pub fn new(count: u32, letter: char) -> Self {
        Self {
            count,
            letter,
        }
    }
}

pub type PatternTree = BTreeMap<String, Vec<Follower>>;

pub struct PatternTrees {
    pattern_trees: Vec<PatternTree>,
}

impl PatternTrees {
    pub fn new(pattern_trees: Vec<PatternTree>) -> Self {
        Self {
            pattern_trees,
        } 
    }

    pub fn statistically_significant(&self, pattern: &str) -> Vec<char> {
        const LESS_PATTERN_LENGTH_FROM_MAX: usize = 6;
        let mut followers: Vec<char> = Vec::with_capacity(20);
        let min_pattern_tree;
        if pattern.len() < LESS_PATTERN_LENGTH_FROM_MAX {
            min_pattern_tree = 0;
        } else {
            min_pattern_tree = self.pattern_trees.len() - LESS_PATTERN_LENGTH_FROM_MAX;
        }
        let max = self.pattern_trees.len().min(pattern.len() + 1);
        //println!("{} {}", max, min_pattern_tree);
        for index in min_pattern_tree..max {
            let index = max - index - 1;
            //println!("{} {}", index, &pattern[index..]);
            if let Some(tree_followers) = self.pattern_trees[index].get(&pattern[..index]) {
                followers.extend(tree_followers.iter().map(|follower| follower.letter));
            }
        }
        followers
    }

    pub fn write_encoding_error_handling(&self, path: Option<&str>) {
        let path: &str = match path {
            Some(path) => path,
            None => "pattern_tree_encoding.txt",
        };
        self.write_encoding(path).unwrap_or_else(|err| eprintln!("{}", err));
    }

    pub fn write_encoding(&self, path: &str) -> Result<(), io::Error>{
        let mut output = File::create(path)?;
        for pattern_tree in self.pattern_trees.iter() {
            for (pattern, followers) in pattern_tree {
                for follower in followers {
                    write!(output, "{}", pattern)?;
                    writeln!(output, "{}{}", follower.letter, follower.count)?;
                }
            }
            writeln!(output, "---")?;
        }
        Ok(())
    }
}