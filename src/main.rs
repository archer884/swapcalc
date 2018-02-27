extern crate chrono;

mod error;
mod sample;
mod summary;

use error::*;

fn main() {
    let path = std::env::args().nth(1).expect("Path not provided");
    if let Err(e) = execute(&path) {
        println!("{}", e);
    }
}

fn execute(path: &str) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use summary::Summary;

    let reader = File::open(path)
        .map(BufReader::new)
        .map_err(|e| Error::io(e, format!("Unable to open path: {}", path)))?;


    let mut summary = Summary::default();
    for line in reader.lines() {
        let line = line.map_err(|e| Error::io(e, "Bad utf-8"))?;
        let sample = line.parse()?;
        summary.apply(&sample);
    }

    println!("{}", summary);
    Ok(())
}
