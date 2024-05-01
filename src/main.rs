const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = todo!();

    for answer in GAMES.split_whitespace() {
        play(answer, guesser);
    }
}
