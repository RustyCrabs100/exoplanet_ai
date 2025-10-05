// This is the location for creating the AI/ML Model

use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::{error::Error, fs::File, io::Write, ops::Range};

/// This is the struct that is outputted after converting the .csv data into Rust data
#[derive(Debug, Deserialize, Clone)]
pub struct Row {
    /// The training data's indentifications
    /// e.g. "candidate", "confirmed", "false positive"
    /// This value is checked by one of the following headers:
    /// "disposition", "tfopwg_disp", or "koi_disposition"
    /// Values that should be in the status variable are:
    /// Candidates: "APC", "PC", "candidate", and "CANDIDATE"
    /// Not a Planet: "FA", "FP", "false positive", and "FALSE POSITIVE"
    /// Exoplanet: "KP", "CP", "confirmed", and "CONFIRMED"
    status: String,
}

impl Row {
    /// Get rid of rows that are candidates
    pub fn row_ok(&self) -> bool {
        self.status != "candidate" && self.status != "CANDIDATE"
    }
}

pub fn call_ai() {
    let _ = read_csv("data\\koi_planets.csv");
    println!("Called the AI");
}

/// Reads the data from the .csv file inputted into this function.
/// Returns a Result, of either any type of error or 2 vectors, exoplanets and not a exoplanet,
/// or a tuple of 2 Vec<Row>'s
/// IMPORTANT NOTICE:
///     All inputted .csv files must have a header.
///     If the header is missing, this function will output broken/weird/unexpected information.
fn read_csv(csv_file: &str) -> Result<(Vec<Row>, Vec<Row>), Box<dyn Error>> {
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
    let mut confirmed: Vec<Row> = Vec::new();
    let mut not_a_planet: Vec<Row> = Vec::new();
    // Iterate
    for result in rdrmb.deserialize::<Row>() {
        let row: Row = result?;
        if !row.row_ok() {
            continue;
        }
        match row.status.to_lowercase().as_str() {
            "confirmed" => confirmed.push(row),
            "false positive" => not_a_planet.push(row),
            other => eprintln!("Unexpected input, {}", other),
        }
    }
    Ok((confirmed, not_a_planet))
}

fn iter_compare_planet_rows(mut true_rows: Vec<Row>, mut false_rows: Vec<Row>) -> Range<f64> {
    todo!()
}
