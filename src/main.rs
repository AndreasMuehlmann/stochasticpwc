use std::env;
use std::collections::VecDeque;

mod pattern_trees;
mod pattern_trees_factory;

use crate::pattern_trees_factory::PatternTreesFactory;
use crate::pattern_trees::PatternTrees;

//TODO: multithreading
//TODO: exploration rate
//TODO: probability of word
//TODO: average of probabilities of words

fn crack_hash_bfs(pattern_trees: PatternTrees, max_len: usize, hash: String) {
    let mut queue: VecDeque<String> = VecDeque::with_capacity(100000);
    queue.push_back("".to_string());
    while !queue.is_empty() {
        let current: String = queue.pop_back().unwrap();
        if current.len() > max_len {
            continue
        }
        if current.starts_with(&hash[..2]) {
            println!("{}", current);
        }
        if current == hash {
            println!("hash {} matches password {}", hash, current);
            return;
        }
        for stat_signif in pattern_trees.statistically_significant(&current).iter() {
            let mut new_password = current.clone();
            new_password.push(*stat_signif);
            queue.push_back(new_password);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern_trees_factory = PatternTreesFactory::new(7);
    let pattern_trees: PatternTrees = pattern_trees_factory.from_paths_error_handling(args);
    println!("built pattern trees");
    // pattern_trees.write_encoding_error_handling(None);
    // pattern_trees.print_probability_distribution();
    let password = "andreas1".to_string();
    crack_hash_bfs(pattern_trees, password.len(), password);
}
