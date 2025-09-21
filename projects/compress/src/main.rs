use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`")
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); // reads the contents of the file
    let output = File::create(args().nth(2).unwrap()).unwrap(); // creates a new empty file handle

    let mut encoder = GzEncoder::new(output, Compression::default()); // Any data written into encoder gets compressed and then written to the output file.
    let start = Instant::now(); // start time
    copy(&mut input, &mut encoder).unwrap(); // Reads from input (source file) and writes into encoder (gzip writer).
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
