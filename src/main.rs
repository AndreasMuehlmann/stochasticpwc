use std::env;

mod pattern_trees;
mod pattern_trees_factory;

use crate::pattern_trees_factory::PatternTreesFactory;
use crate::pattern_trees::PatternTrees;


fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern_trees_factory = PatternTreesFactory::new(5);
    let pattern_trees: PatternTrees = pattern_trees_factory.from_paths_error_handling(args);
    pattern_trees.write_encoding_error_handling(None);
}
