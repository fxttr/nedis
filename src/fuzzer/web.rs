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
        todo!()
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