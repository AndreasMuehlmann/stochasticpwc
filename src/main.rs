use std::collections::VecDeque;

use clap::Parser;

mod pattern_trees;
mod pattern_trees_factory;

use crate::pattern_trees_factory::PatternTreesFactory;
use crate::pattern_trees::PatternTrees;


//TODO: command line arguments
//TODO: multithreading
//TODO: exploration rate
//TODO: probability of word
//TODO: average of probabilities of words


/// Program to crack passwords with probability
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
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

fn crack_hash_bfs(pattern_trees: PatternTrees, max_len: usize, hash: String) -> Option<String> {
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
            return Some(current);
        }
        for stat_signif in pattern_trees.statistically_significant(&current).iter() {
            let mut new_password = current.clone();
            new_password.push(*stat_signif);
            queue.push_back(new_password);
        }
    }
    return None;
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
        eprintln!("ERROR: either option --encoding or --list_passwords has to be given");
        return;
    }
    println!("INFO: Built pattern trees");
    
    if let Some(path_write_probabilities) = args.path_write_probabilities {

        println!("INFO: Writing probabilities...");
        pattern_trees.write_probability_distribution(&path_write_probabilities);
        println!("INFO: Wrote probabilities");
    }
    if let Some(path_write_encoding) = args.path_write_encoding {
        println!("INFO: Writing encoding...");
        pattern_trees.write_encoding(&path_write_encoding);
        println!("INFO: Wrote encoding");
    }
    if let Some(password_hash) = args.password_hash {
        println!("INFO: Attacking...");
        if let Some(password) = crack_hash_bfs(pattern_trees, password_hash.len(), password_hash) {
            println!("DONE: Found {}", password);
        }
        else {
            println!("DONE: Nothing Found");
        }
    }
}
