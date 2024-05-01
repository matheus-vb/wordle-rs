const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = todo!();

    for answer in GAMES.split_whitespace() {
        play(answer, guesser);
    }
}

fn play<G: Guesser>(answer: &'static str, guesser: G) {}

enum Correctness {
    //GREEN
    Correct,

    //YELLOW
    Misplaced,

    //GRAY
    Absent,
}

struct Guess {
    word: String,
    maks: [Correctness; 5],
}

trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
