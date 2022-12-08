use structopt::StructOpt;
use aoc2022::tree_house::Forrest;

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

    let mut forrest = Forrest::new(&content);
    forrest.check_visibility();
  
    let viscount = forrest.count_visable();
    println!("Visible: {viscount:?} ");
}
    