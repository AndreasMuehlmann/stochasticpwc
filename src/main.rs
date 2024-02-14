use std::env;
use std::collections::VecDeque;

mod pattern_trees;
mod pattern_trees_factory;

use crate::pattern_trees_factory::PatternTreesFactory;
use crate::pattern_trees::PatternTrees;

fn crack_hash_bfs(pattern_trees: PatternTrees, hash: String) {
    let mut queue: VecDeque<String> = VecDeque::with_capacity(100000);
    queue.push_back("".to_string());
    // max length of one queue element has to be the max length of a pattern
    while !queue.is_empty() {
        let current: String = queue.pop_front().unwrap();
        println!("{}", current);
        if current == hash {
            println!("hash {} matches password {}", hash, current);
            return;
        }
        for stat_signif in pattern_trees.statistically_significant(&current).iter() {
            let mut new_password = current.clone();
            new_password.push(*stat_signif);
            queue.push_back(new_password);
        }
        // queue = dbg!(queue);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern_trees_factory = PatternTreesFactory::new(5);
    let pattern_trees: PatternTrees = pattern_trees_factory.from_paths_error_handling(args);
    //pattern_trees.write_encoding_error_handling(None);
    // pattern_trees.print_probability_distribution();
    // crack_hash_bfs(pattern_trees, "passw".to_string())
}
