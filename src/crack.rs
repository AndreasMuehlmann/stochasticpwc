use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

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

pub fn crack_mp(pattern_trees: PatternTrees, max_len: usize, hash: String, threads: usize) -> Option<String> {
    let (tx, rx): (Sender<Word>, Receiver<Word>) = mpsc::channel();
    tx.send(Word::new("".to_string(), 1.0)).unwrap();

    let pattern_trees: Arc<PatternTrees> = Arc::new(pattern_trees);

    let mut handles = vec![];
    let mut thread_txs = vec![];
    for _ in 0..threads {
        let tx = tx.clone();
        let (thread_tx, rx): (Sender<Word>, Receiver<Word>) = mpsc::channel();
        thread_txs.push(thread_tx);

        let pattern_trees = Arc::clone(&pattern_trees);
        let hash = hash.clone();

        let handle = thread::spawn(move || {
            loop {
                let current = match rx.recv_timeout(Duration::new(0, 50000000)) {
                    Ok(current) => current,
                    Err(_) => return,
                };

                if current.pattern.starts_with(&hash[0..2]) {println!("{}", current.pattern);}
                if current.pattern == hash { return; }
                if current.pattern.len() >= max_len { continue; }

                for probable_follower in pattern_trees.probable_followers(&current.pattern).iter() {
                    let mut new_password = current.pattern.clone();
                    new_password.push(probable_follower.letter);
                    let result = tx.send(Word::new(new_password, current.probability * probable_follower.probability));
                    if result.is_err() { return; }
                }
            };
        });
        handles.push(handle);
    }
    let mut index = 0;
    loop {
        index = (index + 1) % threads;
        let word = match rx.recv_timeout(Duration::new(0, 50000000)) {
            Ok(word) => word,
            Err(_) => break,
        };
        let result = thread_txs[index].send(word);
        if result.is_err() { break; }
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

        if current.pattern.starts_with(&hash[0..2]) {println!("{}", current.pattern);}
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
