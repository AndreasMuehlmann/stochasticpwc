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

                if line.is_empty() {
                    continue
                }

                let is_ascii = line.is_ascii();
                let mut sub_strings = Self::sub_strings_max_len(line, self.count_pattern_trees, is_ascii);
                sub_strings.reverse();
                for mut sub_string in sub_strings {
                    while !sub_string.is_empty() {
                        let split_sub_string = Self::split_end(sub_string);
                        sub_string = split_sub_string.0;
                        let following_letter = split_sub_string.1;
                        let pattern_length = if is_ascii { sub_string.len() } else { sub_string.chars().count() };
                        Self::insert_kv_pair(&mut pattern_trees[pattern_length], sub_string.clone(), Follower::new(1, following_letter));
                    }
                }
            }
        }
        for pattern_tree in pattern_trees.iter_mut() {
            for followers in pattern_tree.values_mut() {
                // followers.retain(|follower| follower.count != 1);
                followers.sort_by(|a, b| b.count.cmp(&a.count));
            }
        }
        Ok(PatternTrees::new(pattern_trees))
    }

    fn sub_strings_max_len(string: String, max_len: usize, is_ascii: bool) -> Vec<String> {
        let mut sub_strings: Vec<String> = Vec::with_capacity(15);
        if is_ascii {
            let string_char_count = string.len();
            for i in 0..string_char_count {
                sub_strings.push(string[string_char_count - i - 1..string_char_count - i - 1 + max_len.min(i + 1)].to_string());
            }
        } else {
            let string_char_count = string.chars().count();
            for i in 0..string_char_count {
                sub_strings.push(string
                                 .chars()
                                 .skip(string_char_count - i - 1).take(max_len.min(i + 1))
                                 .collect()
                                 );
            }

        }
        sub_strings
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

            let (line, count) = Self::parse_count_from_encoding(line.to_string(), pattern_length);
            let (pattern, following_letter) = Self::split_end(line);
            Self::insert_kv_pair(&mut pattern_tree, pattern, Follower::new(count, following_letter))
        }
        Ok(PatternTrees::new(pattern_trees))
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
            let mut followers = Vec::with_capacity(10);
            followers.push(new_follower);
            pattern_tree.insert(pattern, followers);
        }
    }

    fn split_end(mut string: String) -> (String, char) {
        let following_letter = string.pop().unwrap();
        (string, following_letter)
    }

    fn split_off_chars(mut string: String, cut_off: usize) -> (String, String) {
        let byte_offset = if string.is_ascii() { cut_off } else {
            string
                .char_indices()
                .nth(cut_off)
                .map(|(index, _)| index)
                .unwrap() 
        };
        let off_split = string.split_off(byte_offset);
        (string, off_split)
    }

    fn parse_count_from_encoding(line: String, pattern_length: usize) -> (String, u32) {
        let (line, count) = Self::split_off_chars(line, pattern_length + 1);
        let count = count.parse::<u32>().unwrap();
        (line, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_off_chars() {
        let string: String = "abcde".to_string();
        let (left, right) = PatternTreesFactory::split_off_chars(string, 3);
        assert_eq!(left, "abc".to_string());
        assert_eq!(right, "de".to_string());
        let string: String = "中¡bcde".to_string();
        let (left, right) = PatternTreesFactory::split_off_chars(string, 4);
        assert_eq!(left, "中¡bc".to_string());
        assert_eq!(right, "de".to_string());
    }

    #[test]
    fn test_sub_strings_max_len() {
        let sub_strings = PatternTreesFactory::sub_strings_max_len("abcde".to_string(), 3, true);
        assert_eq!(sub_strings[0], "e".to_string());
        assert_eq!(sub_strings[1], "de".to_string());
        assert_eq!(sub_strings[2], "cde".to_string());
        assert_eq!(sub_strings[3], "bcd".to_string());
        assert_eq!(sub_strings[4], "abc".to_string());
        
    }

}
