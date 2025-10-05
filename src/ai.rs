// This is the location for creating the AI/ML Model

use csv::ReaderBuilder;
use serde_json::{Map, Value, json};
use std::{error::Error, fs::File, io::Write};

pub fn call_ai() {
    let _ = read_csv("data\\koi_planets.csv");
    println!("Called the AI");
}

/// Reads the data from the .csv file inputted into this function.
/// Returns a Result, of either any type of error or a Vector containing each Row keyed by headers.
/// IMPORTANT NOTICE:
///     All inputted .csv files must have a header.
///     If the header is missing, this function will output broken/weird/unexpected information.
fn read_csv(csv_file: &str) -> Result<Vec<Map<String, Value>>, Box<dyn Error>> {
    // Create a refrence counted, dynamically checked for borrow rules, Reader.
    // .csv comments are defined by starting the message with #
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        // 0b00100011 = # in Binary
        .comment(Some(0b00100011))
        .from_path(csv_file)?;
    // Mutably borrow rdr
    let rdrmb = &mut rdr;
    // Take the first row (the headers)
    let headers = rdrmb.headers()?.clone();
    // Create a Vec<> of rows indexed by headers
    let mut rows = Vec::new();
    for result in rdrmb.records() {
        let record = result?;
        let mut obj = serde_json::Map::new();

        for (h, v) in headers.iter().zip(record.iter()) {
            obj.insert(h.to_string(), json!(v));
        }
        rows.push(obj);
    }
    let json = serde_json::to_string_pretty(&rows)?;
    let mut file = File::create("data\\output.json")?;
    file.write_all(json.as_bytes())?;

    println!("Wore {} rows to data/output.json", rows.len());

    Ok(rows)
}
