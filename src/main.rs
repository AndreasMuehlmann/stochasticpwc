use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

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

fn pattern_trees_from_pw_lists(paths: Vec<&str>) -> Result<Vec<PatternTree>, io::Error> {
    let mut pattern_trees: Vec<PatternTree> = vec![PatternTree::new(), PatternTree::new(), PatternTree::new()];
    for path in paths {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
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

            let mut pattern: String = "".to_string();
            //println!("{}", line);
            for letter in line.chars() {
                if pattern.len() > pattern_trees.len() {
                    pattern.remove(0);
                }
                // println!("{}", pattern);
                for index in 0..pattern.len() {
                    let sub_pattern = pattern.clone()[0..index].to_string();
                    let following_letter = pattern.chars().collect::<Vec<char>>()[index];
                    if let Some(followers) = pattern_trees[index].get_mut(&pattern) {
                        for follower in followers.iter() {
                            if follower.letter == letter {
                                let new_follower = Follower::new(follower.count + 1, following_letter);
                                followers.insert(new_follower);
                                break;
                            }
                        }
                    } else {
                        let follower = Follower::new(1, following_letter);
                        let mut followers = BTreeSet::new();
                        followers.insert(follower);

                        // println!("{}, {}", pattern, letter);
                        pattern_trees[index].insert(sub_pattern, followers);
                    }
                }
                pattern.push(letter);
            }
        }
    }
    Ok(pattern_trees)
}

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
    let mut pattern_trees: Vec<PatternTree> = vec![];

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
    //let args: Vec<String> = env::args().collect();
    let mut path = "pattern_tree_encoding.txt".to_string();
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
    write_pattern_trees(pattern_trees, "pattern_tree_reproduction_encoding.txt");
    let pattern_trees = pattern_trees_from_pw_lists(vec!["password_list_short.txt"]).unwrap();
    write_pattern_trees(pattern_trees, "pattern_tree_encoding.txt");
}
