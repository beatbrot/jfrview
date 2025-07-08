use jfrview::Sample;
use std::fs::File;

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 1000)]
fn heavy() -> Sample {
    let f = File::open("test-data/heavy.jfr").unwrap();
    Sample::from_file(f, true, true).unwrap()
}

#[divan::bench(sample_count = 1000)]
fn profiler_multichunk() -> Sample {
    let f = File::open("test-data/profiler-multichunk.jfr").unwrap();
    Sample::from_file(f, true, true).unwrap()
}

#[divan::bench(sample_count = 1000)]
fn recording() -> Sample {
    let f = File::open("test-data/recording.jfr").unwrap();
    Sample::from_file(f, true, true).unwrap()
}

#[divan::bench(sample_count = 1000)]
fn recording_2_1() -> Sample {
    let f = File::open("test-data/recording-2_1.jfr").unwrap();
    Sample::from_file(f, true, true).unwrap()
}
