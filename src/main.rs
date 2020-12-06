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

// Deserialization Function From Slice
fn deserialize_read_str(csv: &str) -> Result<(), Box<dyn Error>> {
    // Instantiate Reader & Row Counter
    let mut r = Reader::from_reader(csv.as_bytes());
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
    // (1) Read From File => "File"
    // (2) Read From String Slice => "Slice"
    let processing_flag: &str = "File";

    // Input CSV String Slice
    let csv: &str = "username,email,address,unique_id,active
    aozerov,ozerovmusic@gmail.com,370 Highway 7E,AO0112,Y
    bozerov,alexei.ozerov.7@gmail.com,370 Highway 7E,AO0113,N";

    // Run function, if error; catch error exit program
    match processing_flag {
        "File" => {
            if let Err(err) = deserialize_read_file() {
                println!("Error Deserializing: {}", err);
                process::exit(1);
            }
        }

        "Slice" => {
            if let Err(err) = deserialize_read_str(&csv) {
                println!("Error Deserializing: {}", err);
                process::exit(1);
            }
        }

        &_ => println!("Error, not a valid option!"),
    }
}
