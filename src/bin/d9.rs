use structopt::StructOpt;
use aoc2022::rope_bridge::RopeModel;

// Read some lines of a file
#[derive(StructOpt)]
struct Cli {
    /// Input file to read
    file: String,
}

fn main()   {
    let args = Cli::from_args();
    let contents = std::fs::read_to_string(&args.file)
      .expect("could not read file");

    let mut rm = RopeModel::new(9);
    rm.run_script(&contents);
    let spots = rm.get_results();
    println!("Spots: {spots} ");
}
    