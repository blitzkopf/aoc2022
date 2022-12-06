use structopt::StructOpt;
use aoc2022::cleanup::Tasklist;

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
    let tl = Tasklist::new(&content);
    let cc = tl.count_contains();
    println!("value: {cc} ");
    let co = tl.count_overlaps();
    println!("value: {co} ");
}
    