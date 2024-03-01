use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

use crate::pattern_trees::PatternTrees;


#[derive(Debug)]
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

pub fn crack_mp(pattern_trees: PatternTrees, max_len: usize, hash: String, threads: usize, batch_size: usize) -> Option<String> {
    let mut queue: VecDeque<Word> = VecDeque::with_capacity(100000);
    queue.push_back(Word::new("".to_string(), 1.0));
    let queue: Arc<Mutex<Option<VecDeque<Word>>>> = Arc::new(Mutex::new(Some(queue)));

    let pattern_trees: Arc<PatternTrees> = Arc::new(pattern_trees);

    let mut handles = vec![];
    for _ in 0..threads {
        let queue = Arc::clone(&queue);
        let pattern_trees = Arc::clone(&pattern_trees);
        let hash = hash.clone();
        let mut result: VecDeque<Word> = VecDeque::with_capacity(batch_size * 10);
        let mut batch: VecDeque<Word> = VecDeque::new();

        let handle = thread::spawn(move || {
            loop {
                batch.clear();
                {
                    let mut queue = queue.lock().unwrap();
                    match &mut *queue {
                        Some(queue) => {
                            if queue.is_empty() {
                                continue;
                            }
                            if queue.len() < batch_size + 1 {
                                batch.append(queue);
                            } else {
                                batch = queue.split_off(batch_size);
                            }
                        },
                        None => return,
                    }
                }
                for current in batch.iter() {
                    println!("{}", current.pattern);
                    //if current.pattern.starts_with(&hash[0..2]) {println!("{}", current.pattern);}
                    if current.pattern == hash { }
                    if current.pattern.len() >= max_len { continue; }
                    for probable_follower in pattern_trees.probable_followers(&current.pattern).iter() {
                        let mut new_password = current.pattern.clone();
                        new_password.push(probable_follower.letter);
                        result.push_back(Word::new(new_password, current.probability * probable_follower.probability));
                    }
                }
                let mut queue = queue.lock().unwrap();
                match &mut *queue {
                    Some(queue) => {
                        queue.append(&mut result);
                    },
                    None => {
                        return;
                    },
                }
                thread::sleep(Duration::new(5, 0));
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
