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