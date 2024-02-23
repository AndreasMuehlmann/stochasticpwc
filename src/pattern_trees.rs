use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, Write};


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct PatternTrees {
    pattern_trees: Vec<PatternTree>,
    probability_distributions: Vec<BTreeMap<u64, f64>>,
    cut_off_counts: Vec<u64>,
}

impl PatternTrees {
    pub fn new(pattern_trees: Vec<PatternTree>) -> Self {
        let mut probability_distributions: Vec<BTreeMap<u64, f64>>= Vec::new();
        let mut cut_off_counts: Vec<u64> = Vec::new();
        for pattern_tree in pattern_trees.iter() {
            let probability_distribution = Self::count_probability_distribution(&pattern_tree);
            cut_off_counts.push(Self::cut_off_count(&probability_distribution));
            probability_distributions.push(probability_distribution);
        }
        Self {
            pattern_trees,
            probability_distributions,
            cut_off_counts,
        } 
    }

    pub fn alphabet(&self) -> Vec<char> {
        self.pattern_trees[0].get("").unwrap().iter().map(|follower| follower.letter).collect()
    }

    pub fn patterns(&self, pattern_tree_index: usize) -> Vec<String> {
       self.pattern_trees[pattern_tree_index].clone().into_keys().collect::<Vec<String>>()
    }

    fn cut_off_count(probability_distribution: &BTreeMap<u64, f64>) -> u64 {
        let mut summed_probability: f64 = 0.0;
        for (count, count_probability) in probability_distribution.iter() {
            summed_probability += count_probability;
            if summed_probability > 0.5 {
                return *count;
            }
        }
        0
    }

    fn count_probability_distribution(pattern_tree: &PatternTree) -> BTreeMap<u64, f64> {
        let mut count_probabilities: BTreeMap<u64, f64> = BTreeMap::new();

        let amount_counts: BTreeMap<u64, u64> = Self::amount_counts(pattern_tree);
        let total_number_of_counts: u64 = amount_counts.values().sum();
        for (count, amount_count) in amount_counts.iter() {
            count_probabilities.insert(*count, (*amount_count) as f64 / total_number_of_counts as f64);
        }
        count_probabilities
    }
    
    fn amount_counts(pattern_tree: &PatternTree) -> BTreeMap<u64, u64> {
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

    pub fn statistically_significant(&self, pattern: &str) -> BTreeSet<char> {
        const LESS_PATTERN_LENGTH_FROM_MAX: usize = 2;
        let mut followers: BTreeSet<char> = BTreeSet::new();

        let max = self.pattern_trees.len().min(pattern.len() + 1);
        let min_pattern_tree = if max < LESS_PATTERN_LENGTH_FROM_MAX { 0 } else { max - LESS_PATTERN_LENGTH_FROM_MAX };

        for index in min_pattern_tree..max {
            if let Some(tree_followers) = self.pattern_trees[index].get(&pattern[pattern.len() - index..]) {
                let probable_followers: Vec<char> = tree_followers
                    .iter()
                    .filter_map(
                        |follower| if follower.count as u64 > self.cut_off_counts[index] {Some(follower.letter)} else {None}
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
        for (index, probability_distribution) in self.probability_distributions.iter().enumerate() {
            writeln!(output, "PATTERNTREE {}", index)?;
            for (count, count_probability) in probability_distribution.iter() {
                write!(output, "{:<8} ", count)?;
                writeln!(output, "{:.8} ", count_probability)?;
            }
            writeln!(output, "\n")?;
        }

        writeln!(output, "CUT OFFS")?;
        for cut_off_count in self.cut_off_counts.iter() {
            write!(output, "{}, ", cut_off_count)?;
        }
        writeln!(output)?;
        Ok(())
    }

    pub fn write_encoding(&self, path: &str) -> Result<(), io::Error> {
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
