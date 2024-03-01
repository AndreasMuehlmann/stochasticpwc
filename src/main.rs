use clap::Parser;

mod pattern_tree;
mod pattern_trees;
mod pattern_trees_factory;
mod crack;

use crate::pattern_trees_factory::PatternTreesFactory;
use crate::pattern_trees::PatternTrees;
use crate::crack::{crack, crack_mp};


//TODO: multithreading batch sizes and with channels for stopping and getting the result
//TODO: target, that supports lists and actual hashes
//TODO: test stochastic pwc against traditional methods


/// Program to crack passwords with probability
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = 1)]
    threads: usize,

    #[arg(short, long, default_value_t = 5)]
    count_pattern_trees: usize,

    #[arg(short, long)]
    encoding: Option<String>,

    #[arg(short, long)]
    password_hash: Option<String>,

    #[arg(short, long)]
    list_passwords: Option<String>,

    #[arg(long)]
    path_write_encoding: Option<String>,

    #[arg(long)]
    path_write_probabilities: Option<String>,
}


fn main() {
    let args: Args = Args::parse();
    let pattern_trees_factory = PatternTreesFactory::new(args.count_pattern_trees);
    let pattern_trees: PatternTrees;

    if let Some(encoding) = args.encoding {
        println!("INFO: Building pattern tree from encoding...");
        pattern_trees = pattern_trees_factory.pattern_trees_with_error_handling(
            PatternTreesFactory::from_encoding, "an encoding".to_string(),  encoding);
    } else if let Some(password_list) = args.list_passwords {
        println!("INFO: Building pattern tree from password list...");
        pattern_trees = pattern_trees_factory.pattern_trees_with_error_handling(
            PatternTreesFactory::from_password_list, "a list of passwords".to_string(), password_list);
    } else {
        pattern_trees = pattern_trees_factory.pattern_trees_with_error_handling(
            PatternTreesFactory::from_encoding, "an encoding".to_string(),  "pattern_tree_encoding.txt".to_string());
    }
    println!("INFO: Built pattern trees");
    
    if let Some(path_write_probabilities) = args.path_write_probabilities {
        println!("INFO: Writing probabilities...");
        pattern_trees.write_with_error_handling(PatternTrees::write_probability_distribution, 
                                                "the probability distribution of the counts of patterns".to_string(), path_write_probabilities);
        println!("INFO: Wrote probabilities");
    }
    if let Some(path_write_encoding) = args.path_write_encoding {
        println!("INFO: Writing encoding...");
        pattern_trees.write_with_error_handling(PatternTrees::write_encoding, 
                                                "the encoding for the pattern trees".to_string(), path_write_encoding);
        println!("INFO: Wrote encoding");
    }
    if let Some(password_hash) = args.password_hash {
        println!("INFO: Attacking...");
        let optional_password = if args.threads == 1 {crack(pattern_trees, password_hash.len(), password_hash)}
            else {crack_mp(pattern_trees, password_hash.len(), password_hash, args.threads, 100)};
        if let Some(password) = optional_password {
            println!("DONE: Found {}", password);
        }
        else {
            println!("DONE: Nothing Found");
        }
    }
}
