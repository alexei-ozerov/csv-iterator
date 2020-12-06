use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::process;

// Define CSV Data Structure
#[derive(Deserialize, Debug)]
struct Record {
    username: String,
    email: String,
    address: String,
    unique_id: String,
    active: String,
}

// Deserialization Function From File
fn deserialize_read_file() -> Result<(), Box<dyn Error>> {
    // Instantiate Reader & Row Counter
    let mut r = Reader::from_path("input.csv")?;
    let mut counter: u32 = 0;

    // Iterate over records, and deserialize into struct
    for row in r.deserialize() {
        let row: Record = row?;
        println!(
            "User: {} Email: {} Address: {} ID: {} Active: {}",
            row.username, row.email, row.address, row.unique_id, row.active
        );
        counter = counter + 1;
    }

    println!("Rows Processed: {}", counter);

    Ok(())
}

fn main() {
    // Run function, if error; catch error exit program
    if let Err(err) = deserialize_read_file() {
        println!("Error Deserializing: {}", err);
        process::exit(1);
    }
}
