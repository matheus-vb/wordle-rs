use wordle_solver::Wordle;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = wordle_solver::algorithms::Naive::new();

        let wordle = Wordle::new();

        wordle_solver::Wordle::play(&wordle, answer, guesser);
    }
}
