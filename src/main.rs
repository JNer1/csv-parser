use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    process,
};

use csv::Reader;

fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_reader(io::stdin());

    let mut output_file = File::create("../output.txt")?;

    let headers = rdr.headers()?;
    let appended_headers: Vec<String> = headers.iter().map(|h| format!("{}\n", h)).collect();

    for header in appended_headers {
        output_file.write_all(header.as_bytes())?
    }

    println!("Output written to output.txt");

    Ok(())
}

fn main() {
    if let Err(err) = parse_csv() {
        println!("Error parsing file: {:?}", err);
        process::exit(1)
    }
}
