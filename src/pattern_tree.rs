use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Follower {
    pub count: u32,
    pub letter: char,
}

impl Follower {
    pub fn new(count: u32, letter: char) -> Self {
        Self {
            count,
            letter,
        }
    }
}

pub type PatternTreeImplementation = BTreeMap<String, Vec<Follower>>;

#[derive(Debug, Clone)]
pub struct PatternTree {
    pub pattern_tree_implementation: PatternTreeImplementation,
    pub total_follower_count: u64,
}

impl PatternTree {
    pub fn new() -> Self {
        Self {
            pattern_tree_implementation: PatternTreeImplementation::new(),
            total_follower_count: 0,
        }
    }

    pub fn insert(&mut self, pattern: &str, new_follower: Follower) {
        self.total_follower_count += new_follower.count as u64;
        if let Some(followers) = self.pattern_tree_implementation.get_mut(pattern) {
            for follower in followers.iter_mut() {
                if follower.letter == new_follower.letter {
                    follower.count += 1; 
                    return;
                }
            }
            followers.push(new_follower);
        } else {
            let mut followers = Vec::with_capacity(10);
            followers.push(new_follower);
            self.pattern_tree_implementation.insert(pattern.to_string(), followers);
        }
    }


    pub fn patterns(&self) -> Vec<String> {
       self.pattern_tree_implementation.clone().into_keys().collect::<Vec<String>>()
    }

    pub fn probability_distribution(&self) -> BTreeMap<u64, f64> {
        let mut count_probabilities: BTreeMap<u64, f64> = BTreeMap::new();

        let amount_counts: BTreeMap<u64, u64> = self.amount_counts();
        let total_number_of_counts: u64 = amount_counts.values().sum();
        for (count, amount_count) in amount_counts.iter() {
            count_probabilities.insert(*count, (*amount_count) as f64 / total_number_of_counts as f64);
        }
        count_probabilities
    }
    
    fn amount_counts(&self) -> BTreeMap<u64, u64> {
        let mut amount_counts: BTreeMap<u64, u64> = BTreeMap::new();
        for followers in self.pattern_tree_implementation.values() {
            for follower in followers.iter() {
                match amount_counts.get_mut(&(follower.count as u64)) {
                    Some(count) => *count += follower.count as u64,
                    None => {amount_counts.insert(follower.count as u64, 1);},
                }
            }
        }
        amount_counts
    }
}
