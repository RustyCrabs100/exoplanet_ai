// Ctr + Shift + B to ubdate


use std::error::Error;
use csv::{ReaderBuilder};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Row {
    // These must match the actual headers in the CSV
    // Adjust the names and types as needed
    status: String,         // e.g. "candidate", "confirmed", "false positive"
    key_value: Option<f64>, // the numeric value to compare; can be missing
}

// Filter: Only rows with a value in `key_value` and not a "candidate"
fn row_ok(row: &Row) -> bool {
    row.key_value.is_some() && row.status != "candidate"
}

fn main() -> Result<(), Box<dyn Error>> {
    // Hardcoded filename
    let filename = "kepler_cumulative_2025.10.4_10.32.16.csv";


    // Open the CSV file with headers
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(filename)?;

    let mut confirmed: Vec<Row> = Vec::new();
    let mut false_positive: Vec<Row> = Vec::new();

    // Deserialize each row
    for result in rdr.deserialize::<Row>() {
        let row: Row = result?;
        if !row_ok(&row) {
            continue;
        }
        match row.status.as_str() {
            "confirmed" => confirmed.push(row),
            "false positive" => false_positive.push(row),
            _ => {
                // Ignore unknown statuses
            }
        }
    }

    let pruned_confirmed = prune_group(confirmed);
    let pruned_false = prune_group(false_positive);

    println!("Pruned Confirmed: {:?}", pruned_confirmed);
    println!("Pruned False Positive: {:?}", pruned_false);

    Ok(())
}

/// Sorts by `key_value`, and reduces the list by keeping only one row from each pair.
/// If `pick_because_higher` is true, keeps the row with the higher value in each pair.
fn prune_group(mut rows: Vec<Row>) -> Vec<Row> {
    rows.sort_by(|a, b| {
        a.key_value.unwrap().partial_cmp(&b.key_value.unwrap()).unwrap()
    });

    let mut current = rows;

    loop {
        if current.len() <= 1 {
            break;
        }
        let mut next_round: Vec<Row> = Vec::new();
        let mut i = 0;

        while i < current.len() - 1 {
            let a = &current[i];
            let b = &current[i + 1];
            let va = a.key_value.unwrap();
            let vb = b.key_value.unwrap();

            let pick_because_higher = true;

            if pick_because_higher {
                if vb >= va {
                    next_round.push(b.clone());
                } else {
                    next_round.push(a.clone());
                }
            } else {
                if va <= vb {
                    next_round.push(a.clone());
                } else {
                    next_round.push(b.clone());
                }
            }

            i += 2;
        }

        if i == current.len() - 1 {
            // Carry over last unpaired row
            next_round.push(current[i].clone());
        }

        current = next_round;
    }

    current
}



