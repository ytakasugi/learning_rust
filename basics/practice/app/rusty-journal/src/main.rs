use structopt::StructOpt;

mod cli;
mod task;

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
