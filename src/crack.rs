use std::collections::VecDeque;
use std::thread;
use std::sync::{Mutex, Arc};

use crate::pattern_trees::PatternTrees;

pub fn crack_mp(pattern_trees: PatternTrees, max_len: usize, hash: String, threads: usize) -> Option<String> {
    let mut queue: VecDeque<String> = VecDeque::with_capacity(100000);
    queue.extend(pattern_trees.patterns(2));
    let queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(queue));

    let pattern_trees: Arc<PatternTrees> = Arc::new(pattern_trees);

    let mut handles = vec![];
    for _ in 0..threads {
        let queue = Arc::clone(&queue);
        let pattern_trees = Arc::clone(&pattern_trees);
        let hash = hash.clone();

        let handle = thread::spawn(move || {
            loop {
                let current: String;
                {
                    let mut queue = queue.lock().unwrap();
                    if queue.is_empty() {
                        break;
                    }
                    current = queue.pop_back().unwrap();
                }
                if current.len() > max_len {
                    continue;
                }
                let mut own_queue: VecDeque<String> = VecDeque::with_capacity(100);
                own_queue.push_back(current.to_string());
                while !own_queue.is_empty() {
                    let current: String = own_queue.pop_back().unwrap();
                    if current.len() > max_len {
                        continue
                    }
                    if current.starts_with(&hash[..4]) {
                        println!("{}", current);
                    }
                    for stat_signif in pattern_trees.statistically_significant(&current).iter() {
                        let mut new_password = current.clone();
                        new_password.push(*stat_signif);
                        own_queue.push_back(new_password);
                    }
                }
            };
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    return Some("string".to_string());
}

pub fn crack(pattern_trees: PatternTrees, max_len: usize, hash: String) -> Option<String> {
    let mut queue: VecDeque<String> = VecDeque::with_capacity(100000);
    queue.push_back("".to_string());
    while !queue.is_empty() {
        let current: String = queue.pop_back().unwrap();
        if current.len() > max_len {
            continue
        }
        if current.starts_with(&hash[..4]) {
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
