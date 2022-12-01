use structopt::StructOpt;
use aoc2022::calorie_counter::CalorieCollector;

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
    let mut cc = CalorieCollector::new();
    cc.read_input(&content);
    let cals = cc.get_biggest();
    println!("Biggest: {cals}");
    let (e0,e1,e2) = cc.top_3();
    let res=e0+e1+e2;
    println!("Biggest3: {e0}+{e1}+{e2} = {res}");
}