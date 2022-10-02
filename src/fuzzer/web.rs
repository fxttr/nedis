use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::connpool::Connpool;
use crate::fuzzer::Fuzzer;

pub struct WebEnumerate {
    source: String,
    target: String,
    verbose: bool
}

pub struct WebFuzzer {
    target: String,
    verbose: bool
}

impl Fuzzer for WebEnumerate {
    fn new(source: &str, target: &str, verbose: bool) -> Self {
        WebEnumerate {
            source: source.to_owned(),
            target: target.to_owned(),
            verbose
        }
    }

    fn run(self) {
        let file = File::open(self.source).expect("No such file");
        let buf = BufReader::new(file);
        let dirs: Vec<String> = buf.lines()
                                .map(|l| l.expect("Could not parse line"))
                                .collect();
        let pool = Connpool::new(&self.target, self.verbose);
        let concurrent = 8;
        let chunks = dirs.chunks(concurrent);
        let chunk_len = chunks.len();

        for (i, chunk) in chunks.enumerate() {
            pool.bulk(chunk.to_vec());
            print!("Progress: {:.3} %\r", ((i as f32 / chunk_len as f32) * 100 as f32));
            std::io::stdout().flush();
        }
    }
}

impl Fuzzer for WebFuzzer {
    fn new(source: &str, target: &str, verbose: bool) -> Self {
        WebFuzzer {
            target: target.to_owned(),
            verbose
        }
    }

    fn run(self) {
        todo!()
    }
}