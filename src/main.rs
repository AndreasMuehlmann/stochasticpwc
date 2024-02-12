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

fn insert_kv_pair(pattern_tree: &mut PatternTree, pattern: String, mut new_follower: Follower) {
    if let Some(followers) = pattern_tree.get_mut(&pattern) {
        for follower in followers.iter() {
            if follower.letter == new_follower.letter {
                new_follower.count += follower.count;
                followers.insert(new_follower);
                break;
            }
        }
    } else {
        let mut followers = BTreeSet::new();
        followers.insert(new_follower);
        pattern_tree.insert(pattern, followers);
    }
}

fn pattern_trees_from_pw_lists(paths: &[String]) -> Result<Vec<PatternTree>, io::Error> {
    const COUNT_PATTERN_TREES: usize = 3;
    let mut pattern_trees: Vec<PatternTree> = vec![];
    for _ in 0..COUNT_PATTERN_TREES {
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

            for _ in line.chars() {
                let mut tail: String = line.to_string();
                for pattern_length in 0..tail.len().min(COUNT_PATTERN_TREES) {
                    let (pattern, following_letter) = split_kv_pair(&tail, pattern_length);
                    insert_kv_pair(&mut pattern_trees[pattern_length], pattern, Follower::new(1, following_letter));
                    tail = tail[1..].chars().collect();
                }
            }
        }
    }
    Ok(pattern_trees)
}

fn parse_pattern_trees(path: &str) -> Result<Vec<PatternTree>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut pattern_length: usize = 0;
    let mut pattern_tree: PatternTree = BTreeMap::new();
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
            pattern_tree = BTreeMap::new();
            pattern_length += 1;
            continue;
        }

        let (pattern, following_letter) = split_kv_pair(line, pattern_length);

        let count = count_from_encoding(line, pattern_length);
        insert_kv_pair(&mut pattern_tree, pattern, Follower::new(count, following_letter))
    }
    Ok(pattern_trees)
}

fn write_pattern_trees(pattern_trees: Vec<PatternTree>, path: &str) {
    for pattern_tree in pattern_trees {
        for (pattern, followers) in pattern_tree {
            for follower in followers {
                print!("{}", pattern);
                println!("{}{}", follower.letter, follower.count);
            }
        }
        println!("---");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern_trees: Vec<PatternTree>;
    if args.len() > 1 {
        pattern_trees = match pattern_trees_from_pw_lists(&args[1..]) {
            Ok(pattern_trees) => pattern_trees,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        }
    } else {
        let mut path = "pattern_tree_encoding.txt".to_string();
        pattern_trees = loop {
            match parse_pattern_trees(&path) {
                Ok(pattern_trees) => break pattern_trees,
                Err(err) => {
                    eprintln!("Error opening file: {}", err);
                    println!("input a valid file path for a pattern tree encoding");
                    io::stdin()
                        .read_line(&mut path)
                        .expect("Failed to read from stdin");
                }
            }
        }
    }
    write_pattern_trees(pattern_trees, "pattern_tree_reproduction_encoding.txt");
}
