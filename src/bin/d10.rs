use structopt::StructOpt;
use aoc2022::crt::CPU;
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

    let mut cpu = CPU::new(&content);
    let result = cpu.get_results(); 
    println!("Result: {result} ");
}
    