use std::collections::VecDeque;
use std::thread;
use std::sync::{Mutex, Arc};

use crate::pattern_trees::PatternTrees;


pub struct Word {
    pub pattern: String,
    pub probability: f64,
}

impl Word {
    pub fn new(pattern: String, probability: f64) -> Self {
        Self {
            pattern,
            probability,
        }
    }
}

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
                    /*
                    for stat_signif in pattern_trees.statistically_significant(&current).iter() {
                        let mut new_password = current.clone();
                        new_password.push(*stat_signif);
                        own_queue.push_back(new_password);
                    }
                    */
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
    let mut queue: VecDeque<Word> = VecDeque::with_capacity(100000);
    queue.push_back(Word::new("".to_string(), 1.0));
    let mut probabilities: Vec<f64> = (0..max_len).map(|_| 0.0).collect();
    while !queue.is_empty() {
        let current: Word = queue.pop_back().unwrap();

        if current.pattern.starts_with(&hash[..3]) { println!("{}", current.pattern); }
        if current.pattern == hash { return Some(current.pattern); }
        if current.pattern.len() >= max_len { continue; }
        let mut iir_faktor = 0.9;
        if probabilities[current.pattern.len()] > current.probability { 
            iir_faktor = 0.7;
            probabilities[current.pattern.len()] =  iir_faktor * probabilities[current.pattern.len()]
                + (1.0 - iir_faktor) * current.probability;
            continue;
        }
        probabilities[current.pattern.len()] =  iir_faktor * probabilities[current.pattern.len()]
            + (1.0 - iir_faktor) * current.probability;
        if probabilities[current.pattern.len()] > current.probability { continue; }

        for probable_follower in pattern_trees.probable_followers(&current.pattern).iter() {
            let mut new_password = current.pattern.clone();
            new_password.push(probable_follower.letter);
            queue.push_back(Word::new(new_password, current.probability * probable_follower.probability));
        }
    }
    return None;
}
