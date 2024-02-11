use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Follower {
    count: u32,
    letter: char,
}

impl Follower {
    pub fn new(count: u32, letter: char) -> Self {
        Self {
            count,
            letter,
        }
    }
}

type PatternTree = BTreeMap<String, BTreeSet<Follower>>;

fn parse_kv_pair(line: &str) -> (String, Follower) {
    let mut pattern = "".to_string();
    let mut count = "".to_string();
    let mut last_letter = '\0';
    for letter in line.chars() {
        if letter.is_digit(10) {
            count.push(letter);
        } else {
            if last_letter != '\0' {
                pattern.push(last_letter);
            }
            last_letter = letter;
        }
    }
    let following_letter = last_letter;
    let count: u32 = match count.parse::<u32>() {
        Ok(count) => count,
        Err(err) => {
            eprintln!("Error parsing count: {}", err);
            0
        },
    };
    (pattern, Follower::new(count, following_letter))
}

fn parse_pattern_trees(path: &str) -> Result<Vec<PatternTree>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut pattern_length: usize = 0;
    let mut pattern_tree: PatternTree = BTreeMap::new();
    let mut pattern_trees: Vec<PatternTree>= vec![];

    for line in reader.lines() {
        let line = match line {
            Ok(line_content) => line_content,
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                continue;
            }
        };

        let line = line.trim();
        if line.is_empty() {
            continue
        }
        if line == "---" {
            pattern_trees.push(pattern_tree);
            pattern_tree = BTreeMap::new();
            pattern_length += 1;
            continue;
        }

        let (pattern, follower) = parse_kv_pair(line);
        if pattern.len() != pattern_length {
            eprintln!("Wrong pattern length");
        }

        if let Some(followers) = pattern_tree.get_mut(&pattern) {
            followers.insert(follower);
        } else {
            let mut followers = BTreeSet::new();
            followers.insert(follower);
            pattern_tree.insert(pattern, followers);
        }
    }
    Ok(pattern_trees)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path: String = match args.get(1) {
        Some(arg) => arg.clone(),
        None => "".to_string(),
    };
    let pattern_trees: Vec<PatternTree> = loop {
        match parse_pattern_trees(&path) {
            Ok(pattern_trees) => break pattern_trees,
            Err(err) => {
                eprintln!("Error opening file: {}", err);
                println!("input a valid file path");
                io::stdin()
                    .read_line(&mut path)
                    .expect("Failed to read from stdin");
            }
        };
    };
    for pattern_tree in pattern_trees {
        for (pattern, followers) in pattern_tree {
                println!("\"{}\":", pattern);
            for follower in followers {
                println!("  \"{}\" {}", follower.letter, follower.count);
            }
        }
    }
}
