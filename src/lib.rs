use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("the lines consist of word + space + frequency")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, guesser: G) -> Option<usize> {
        let mut history = Vec::new();

        for i in 1..=32 {
            let guess = guesser.guess(&history);

            if guess == answer {
                return Some(i);
            }

            assert!(self.dictionary.contains(&*guess));

            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            })
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    //GREEN
    Correct,

    //YELLOW
    Misplaced,

    //GRAY
    Absent,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Absent; 5];

        // Check correct letters
        for (i, (j, k)) in answer.chars().zip(guess.chars()).enumerate() {
            if j == k {
                c[i] = Correctness::Correct;
            }
        }

        // Check misplaced letters
        let mut used = [false; 5];

        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue; //Already marked as correct
            }

            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&self, history: &[Guess]) -> String;
}

impl Guess {
    pub fn matches(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);

        let mut used = [false; 5];

        //check greens
        for (i, ((prev_guess, &cur_mask), cur_word)) in self
            .word
            .chars()
            .zip(&self.mask)
            .zip(word.chars())
            .enumerate()
        {
            if cur_mask == Correctness::Correct {
                if prev_guess != cur_word {
                    return false;
                } else {
                    used[i] = true;
                }
            }
        }

        //check yellows
        for (i, (cur_word, &cur_mask)) in word.chars().zip(&self.mask).enumerate() {
            if cur_mask == Correctness::Correct {
                continue; // have to be correct, or it would have returned in the earlier loop
            }

            let mut plausible = true;
            if self
                .word
                .chars()
                .zip(&self.mask)
                .enumerate()
                .any(|(j, (g, m))| {
                    if g != cur_word {
                        return false;
                    }

                    if used[j] {
                        return false;
                    }

                    // looking at an 'w' in 'word', and have found an 'w' in the previous guess
                    // the color of that previous 'w' will tell whether this 'w' might be ok
                    match m {
                        Correctness::Correct => {
                            unreachable!("correct guesses should have result in return or be used")
                        }
                        Correctness::Misplaced if j == i => {
                            // 'w' was yellow in the same position las time, so
                            // 'word' cannot be the answer
                            plausible = false;
                            return false;
                        }
                        Correctness::Misplaced => {
                            used[j] = true;
                            return true;
                        }
                        Correctness::Absent => {
                            plausible = false;
                            return false;
                        }
                    }
                })
                && plausible
            {
                // the 'w' character was yellow in the previous guess
            } else if !plausible {
                return false;
            } else {
                // no information about character 'w', so word might still match
            }
        }
        true
    }
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&self, $history: &[crate::Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests {
    mod game {
        use crate::Wordle;

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "moved".to_string() });

            assert_eq!(w.play("moved", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }

                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn wrong() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_string() });

            assert_eq!(w.play("right", guesser), None);
        }
    }

    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => { Correctness::Correct };
            (M) => { Correctness::Misplaced };
            (A) => { Correctness::Absent };
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn all_correct() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C],);
        }

        #[test]
        fn all_misplaced() {
            assert_eq!(Correctness::compute("abcde", "badec"), mask![M M M M M],);
        }

        #[test]
        fn all_wrong() {
            assert_eq!(Correctness::compute("abcde", "fffff"), mask![A A A A A],);
        }

        #[test]
        fn repeat_correct() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C A A A],);
        }

        #[test]
        fn repeat_misplaced() {
            assert_eq!(Correctness::compute("aabbb", "bbcca"), mask![M M A A M],);
        }

        #[test]
        fn mix_correct() {
            assert_eq!(Correctness::compute("abcde", "abfed"), mask![C C A M M],);
        }

        #[test]
        fn repeated_after_misplace() {
            assert_eq!(Correctness::compute("abbab", "aaacc"), mask![C M A A A],);
        }

        #[test]
        fn mix_abscent() {
            assert_eq!(Correctness::compute("baccc", "aaddd"), mask![A C A A A],);
        }
    }
}
