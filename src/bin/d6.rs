use structopt::StructOpt;
use aoc2022::comm_system::find_marker;

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

    let marker = find_marker(&content,4);
    let marker2 = find_marker(&content,14);
    println!("Marker: {marker:?} ");
    println!("Marker2: {marker2:?} ");
}
    