use std::fs::File;
use std::io::{self, Write};

use crate::pattern_tree::PatternTree;


#[derive(Debug, Clone)]
pub struct ProbableFollower {
    pub letter: char,
    pub probability: f64,
}

impl ProbableFollower {
    pub fn new(letter: char, probability: f64) -> Self {
        Self {
            letter,
            probability,
        }
    }
}

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
        self.pattern_trees[0].pattern_tree_implementation.get("")
            .unwrap()
            .iter()
            .map(|follower| follower.letter)
            .collect()
    }

    pub fn patterns(&self, pattern_tree_index: usize) -> Vec<String> {
       self.pattern_trees[pattern_tree_index].patterns()
    }

    pub fn probable_followers(&self, pattern: &str) ->  Vec<ProbableFollower> {
        let less_pattern_length_from_max: usize = self.pattern_trees.len() - 1;
        let mut probable_followers: Vec<ProbableFollower> = self.alphabet()
            .iter()
            .map(|letter| ProbableFollower::new(*letter, 0.0))
            .collect();

        let max = self.pattern_trees.len().min(pattern.len() + 1);
        let min_pattern_tree = if max < less_pattern_length_from_max { 0 } else { max - less_pattern_length_from_max };

        for index in min_pattern_tree..max {
            if let Some(tree_followers) = self.pattern_trees[index].pattern_tree_implementation.get(&pattern[pattern.len() - index..]) {
                for tree_follower in tree_followers.iter() {
                    let probable_follower = probable_followers.iter_mut()
                        .find(|probable_follower| probable_follower.letter == tree_follower.letter)
                        .unwrap();
                    probable_follower.probability += tree_follower.count as f64 
                        / self.pattern_trees[index].total_follower_count as f64 
                        / (max - min_pattern_tree) as f64;
                }
            }
        }
        probable_followers.sort_unstable_by(
            |a, b| b.probability.partial_cmp(&a.probability).unwrap()
            );
        probable_followers.into_iter()
            .take(Self::followers_for_pattern_length(pattern.len()))
            .collect()
    }

    fn followers_for_pattern_length(length: usize) -> usize {
        60 / (length + 1) + 1
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
