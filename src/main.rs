mod fuzzer;

use clap::Parser;
use crate::fuzzer::Fuzzer;

use crate::fuzzer::web::{WebEnumerate, WebFuzzer};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    source: String,

    #[clap(short, long)]
    target: String,

    #[clap(short, long)]
    random: bool,

    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let fuzzer: dyn Fuzzer = match args.random {
        true => WebFuzzer::new(args.source, args.target, args.verbose),
        _ => WebEnumerate::new(args.source, args.target, args.verbose)
    };
}
