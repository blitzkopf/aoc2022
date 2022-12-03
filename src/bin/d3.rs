use structopt::StructOpt;
use aoc2022::rucksacks::RSCollection;

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
    let mut rsc = RSCollection::new();
    rsc.read_input(&content);

    let value = rsc.get_value();
    println!("value: {value} ");

    let value2 = rsc.get_value2();
    println!("value: {value2} ");
}