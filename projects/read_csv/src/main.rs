use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?; // Creates a csv::Reader from the given file path.

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./reports.csv") {
        // checks if the function returned an error.
        // Err(e) is just a value of type Result::Err, holding an error.
        // It doesn’t “throw” anything (Rust doesn’t have exceptions). It just represents an error result.
        // if let lets you check whether a value matches a pattern
        eprintln!("{}", e);
    }
}
