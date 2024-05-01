pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, guesser: G) -> Option<usize> {
    let mut history = Vec::new();

    for i in 1.. {
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
        todo!();
    }
}

pub struct Guess {
    pub word: String,
    pub maks: [Correctness; 5],
}

pub trait Guesser {
    fn guess(self, history: &[Guess]) -> String;
}
