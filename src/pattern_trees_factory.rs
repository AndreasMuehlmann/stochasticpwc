use std::fs::File;
use std::io::{self, BufReader, BufRead};

use crate::pattern_tree::{PatternTree, Follower};
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

    pub fn pattern_trees_with_error_handling(&self, parser: fn(&PatternTreesFactory, &str) -> Result<PatternTrees,
        io::Error>, expected_content: String, mut path: String) -> PatternTrees { 
        let pattern_trees = loop {
            match parser(self, &path) {
                Ok(pattern_trees) => break pattern_trees,
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
        pattern_trees
    }

    pub fn from_password_list(&self, path: &str) -> Result<PatternTrees, io::Error> {
        let mut pattern_trees: Vec<PatternTree> = vec![];
        for _ in 0..self.count_pattern_trees {
            pattern_trees.push(PatternTree::new());
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = match line {
                Ok(line_content) => line_content,
                Err(err) => {
                    eprintln!("ERROR: Reading line in pattern_trees_from_pw_lists: {}", err);
                    continue;
                }
            };

            if line.is_empty() || !line.is_ascii(){
                continue
            }

            let mut sub_strings = Self::sub_strings_max_len(line, self.count_pattern_trees);
            sub_strings.reverse();
            for mut sub_string in sub_strings {
                while !sub_string.is_empty() {
                    let split_sub_string = Self::split_end(sub_string);
                    sub_string = split_sub_string.0;
                    let following_letter = split_sub_string.1;
                    let pattern_length = sub_string.len();
                    pattern_trees[pattern_length].insert(&sub_string, Follower::new(1, following_letter));
                }
            }
        }
        pattern_trees[0].pattern_tree_implementation.get_mut("")
            .unwrap()
            .sort_unstable_by(|a, b| b.count.cmp(&a.count));

        Ok(PatternTrees::new(pattern_trees))
    }

    fn sub_strings_max_len(string: String, max_len: usize) -> Vec<String> {
        let mut sub_strings: Vec<String> = Vec::with_capacity(10);
        let string_char_count = string.len();
        for i in 0..string_char_count {
            sub_strings.push(string[string_char_count - i - 1..string_char_count - i - 1 + max_len.min(i + 1)].to_string());
        }
        sub_strings
    }

    pub fn from_encoding(&self, path: &str) -> Result<PatternTrees, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut pattern_length: usize = 0;
        let mut pattern_tree: PatternTree = PatternTree::new();
        let mut pattern_trees: Vec<PatternTree> = vec![];

        for line in reader.lines() {
            let line = match line {
                Ok(line_content) => line_content,
                Err(err) => {
                    eprintln!("ERROR: Reading line in parse_pattern_trees: {}", err);
                    continue;
                }
            };

            let line = line.trim();
            if line.is_empty() || !line.is_ascii(){
                continue
            }
            if line == "---" {
                pattern_trees.push(pattern_tree);
                pattern_tree = PatternTree::new();
                pattern_length += 1;
                if pattern_length > self.count_pattern_trees - 1 {
                    return Ok(PatternTrees::new(pattern_trees));
                }
                continue;
            }

            let (line, count) = Self::parse_count_from_encoding(line.to_string(), pattern_length);
            let (pattern, following_letter) = Self::split_end(line);
            pattern_tree.insert(&pattern, Follower::new(count, following_letter));
        }
        Ok(PatternTrees::new(pattern_trees))
    }


    fn split_end(mut string: String) -> (String, char) {
        let following_letter = string.pop().unwrap();
        (string, following_letter)
    }

    fn split_off_chars(mut string: String, cut_off: usize) -> (String, String) {
        let off_split = string.split_off(cut_off);
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
    }

    #[test]
    fn test_sub_strings_max_len() {
        let sub_strings = PatternTreesFactory::sub_strings_max_len("abcde".to_string(), 3);
        assert_eq!(sub_strings[0], "e".to_string());
        assert_eq!(sub_strings[1], "de".to_string());
        assert_eq!(sub_strings[2], "cde".to_string());
        assert_eq!(sub_strings[3], "bcd".to_string());
        assert_eq!(sub_strings[4], "abc".to_string());
        
    }

}
