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
#[command(version, about, long_about = None, arg_required_else_help(false))]
struct Args {
    #[arg(short, long)]
    encoding: Option<String>,

    #[arg(short, long)]
    password_hash: Option<String>,

    #[arg(short, long)]
    list_passwords: Option<String>,

    #[arg(long)]
    encoding_path: Option<String>,

    #[arg(long)]
    probabilities_path: Option<String>,
}

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
    let args: Args = Args::parse();
    let pattern_trees_factory = PatternTreesFactory::new(7);
    let pattern_trees: PatternTrees;
    if let Some(list_passwords) = args.list_passwords {
        pattern_trees = pattern_trees_factory.from_paths_error_handling(vec![list_passwords]);
    } else {
        pattern_trees = pattern_trees_factory.from_paths_error_handling(vec![]);
    }
    if let Some(encoding_path) = args.encoding_path {
        pattern_trees.write_encoding_error_handling(&encoding_path);
    }

    println!("built pattern trees");
    
    if let Some(probabilities_path) = args.probabilities_path {
        pattern_trees.write_probability_distribution(&probabilities_path);
    }

    if let Some(password_hash) = args.password_hash {
        crack_hash_bfs(pattern_trees, password_hash.len(), password_hash);
    }
}
