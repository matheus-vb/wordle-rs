use std::collections::HashMap;

use crate::{Guess, Guesser, DICTIONARY};

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("lines should consist of word + space + frequency");
                let count: usize = count.parse().expect("count should be a number");
                (word, count)
            })),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            //update self.remaingin based on history
            self.remaining.retain(|word, _| last.matches(word));
        }

        let mut best: Option<Candidate> = None;

        for (&word, &count) in &self.remaining {
            //compute goodness
            let goodness = 0.0;
            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate {
                        word,
                        count,
                        goodness,
                    })
                }
            } else {
                best = Some(Candidate {
                    word,
                    count,
                    goodness,
                })
            }
        }

        best.unwrap().word.to_string()
    }
}
