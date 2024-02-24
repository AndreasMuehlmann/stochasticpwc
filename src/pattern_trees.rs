use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, Write};

use crate::pattern_tree::PatternTree;


#[derive(Debug, Clone)]
pub struct PatternTrees {
    pattern_trees: Vec<PatternTree>,
}

impl PatternTrees {
    pub fn new(pattern_trees: Vec<PatternTree>) -> Self {
        Self {
            pattern_trees,
        } 
    }

    pub fn alphabet(&self) -> Vec<char> {
        self.pattern_trees[0].pattern_tree_implementation.get("").unwrap().iter().map(|follower| follower.letter).collect()
    }

    pub fn patterns(&self, pattern_tree_index: usize) -> Vec<String> {
       self.pattern_trees[pattern_tree_index].patterns()
    }

    pub fn statistically_significant(&self, pattern: &str) -> BTreeSet<char> {
        const LESS_PATTERN_LENGTH_FROM_MAX: usize = 2;
        let mut followers: BTreeSet<char> = BTreeSet::new();

        let max = self.pattern_trees.len().min(pattern.len() + 1);
        let min_pattern_tree = if max < LESS_PATTERN_LENGTH_FROM_MAX { 0 } else { max - LESS_PATTERN_LENGTH_FROM_MAX };

        for index in min_pattern_tree..max {
            if let Some(tree_followers) = self.pattern_trees[index].pattern_tree_implementation.get(&pattern[pattern.len() - index..]) {
                let probable_followers: Vec<char> = tree_followers
                    .iter()
                    .filter_map(
                        |follower| Some(follower.letter)
                    ).collect();
                followers.extend(probable_followers);
            }
        }
        followers
    }

    pub fn write_with_error_handling(&self, write_function: fn(&PatternTrees, &str) -> Result<(),
        io::Error>, expected_content: String, mut path: String) {
        loop {
            match write_function(self, &path) {
                Ok(()) => return,
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    println!("Previous path {}", path);
                    println!("Input a valid file path, that contains {}", expected_content);
                    path.clear();
                    io::stdin()
                        .read_line(&mut path)
                        .expect("Failed to read from stdin");
                }
            }
        };
    }

    pub fn write_probability_distribution(&self, path: &str) -> Result<(), io::Error> {
        let mut output = File::create(path)?;
        writeln!(output, "PROBABILITY DISTRIBUTIONS")?;
        for (index, pattern_tree) in self.pattern_trees.iter().enumerate() {
            writeln!(output, "PATTERNTREE {}", index)?;
            for (count, count_probability) in pattern_tree.probability_distribution().iter() {
                write!(output, "{:<8} ", count)?;
                writeln!(output, "{:.8} ", count_probability)?;
            }
            writeln!(output, "\n")?;
        }
        Ok(())
    }

    pub fn write_encoding(&self, path: &str) -> Result<(), io::Error> {
        let mut output = File::create(path)?;
        for pattern_tree in self.pattern_trees.iter() {
            for (pattern, followers) in pattern_tree.pattern_tree_implementation.iter() {
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
