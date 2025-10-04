// This is the location for creating the AI/ML Model

use csv::ReaderBuilder;
use serde_json::json;
use std::{cell::RefCell, error::Error, fs::File, io::Write, rc::Rc};

pub fn call_ai() {
    read_csv("data\\koi_planets.csv").expect("Failed to read the csv");
    println!("Called the AI");
}

fn read_csv(csv_file: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Rc::new(RefCell::new(
        ReaderBuilder::new()
            .has_headers(true)
            .comment(Some(0b00100011))
            .from_path(csv_file)?,
    ));
    let mut rdrmb = rdr.borrow_mut();
    let headers = rdrmb.headers()?.clone();
    let mut rows = Vec::new();
    for result in rdrmb.records() {
        let record = result?;
        let mut obj = serde_json::Map::new();

        for (h, v) in headers.iter().zip(record.iter()) {
            obj.insert(h.to_string(), json!(v));
        }
        rows.push(obj);
        println!("{record:?}");
    }
    let json = serde_json::to_string_pretty(&rows)?;
    let mut file = File::create("data\\output.json")?;
    file.write_all(json.as_bytes())?;

    println!("Wore {} rows to data/output.json", rows.len());

    Ok(())
}
