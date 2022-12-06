use structopt::StructOpt;
use aoc2022::supply_stacks::Crane;

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
    let mut  cr = Crane::new(&content);
    let res = cr.run();
    println!("value: {res} ");
}
    