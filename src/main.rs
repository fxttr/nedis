mod fuzzer;
mod connpool;

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

    if args.random {
        let fuzzer = WebFuzzer::new(&args.source, &args.target, args.verbose);

        fuzzer.run();
    } else {
        let fuzzer = WebEnumerate::new(&args.source, &args.target, args.verbose);

        fuzzer.run();
    }
}
