use structopt::StructOpt;
use aoc2022::rps::Game;

// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}

fn main()   {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.file)
      .expect("could not read file");
    let mut game = Game::new();
    game.read_input(&content);
    let score = game.score();
    println!("Score: {} {}",score.0,score.1);
    let score2 = game.score2();
    println!("Score2: {} {}",score2.0,score2.1);
}