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
    //expected_values: Vec<u32>,
    //standard_deviations: Vec<u32>,
}

impl PatternTrees {
    pub fn new(pattern_trees: Vec<PatternTree>) -> Self {
        let expected_values: Vec<u32> = Vec::new();
        let standard_deviations: Vec<u32> = Vec::new();
        for pattern_tree in pattern_trees.iter() {

        }
        Self {
            pattern_trees,
        } 
    }

    fn standard_deviation(count_probabilities: BTreeMap<u64, f64>, expected_value: f64) -> f64 {
        let mut variance: f64 = 0.0;
        for (count, count_probability) in count_probabilities.iter() {
            variance += (*count as f64 - expected_value).powi(2) * count_probability;
        }
        variance
    }

    fn expected_value(count_probabilities: BTreeMap<u64, f64>) -> f64 {
        let mut expected_value: f64 = 0.0;
        for (count, count_probability) in count_probabilities.iter() {
            expected_value += *count as f64 * count_probability;
        }
        expected_value
    }

    fn count_probabilities(pattern_tree: PatternTree) -> BTreeMap<u64, f64> {
        let mut count_probabilities: BTreeMap<u64, f64> = BTreeMap::new();

        let amount_counts: BTreeMap<u64, u64> = Self::amount_counts(pattern_tree);
        let total_number_of_counts: u64 = amount_counts.values().sum();
        for (count, amount_count) in amount_counts.iter() {
            count_probabilities.insert(*count, (*amount_count / total_number_of_counts) as f64);
        }
        count_probabilities
    }
    
    fn amount_counts(pattern_tree: PatternTree) -> BTreeMap<u64, u64> {
        let mut amount_counts: BTreeMap<u64, u64> = BTreeMap::new();
        for followers in pattern_tree.values() {
            for follower in followers.iter() {
                match amount_counts.get_mut(&(follower.count as u64)) {
                    Some(count) => *count += follower.count as u64,
                    None => {amount_counts.insert(follower.count as u64, 1);},
                }
            }
        }
        amount_counts
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
