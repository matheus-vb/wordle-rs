pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, guesser: G) -> Option<usize> {
    let mut history = Vec::new();

    for i in 1..=32 {
        let guess = guesser.guess(&history);

        if guess == answer {
            return Some(i);
        }

        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: guess,
            maks: correctness,
        })
    }
    None
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
    pub maks: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&self, history: &[Guess]) -> String;
}

#[cfg(test)]
mod tests {
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
