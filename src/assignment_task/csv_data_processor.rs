use csv::{ReaderBuilder, Writer};
use std::{error::Error, io};
use std::fs::File;
use serde::{Deserialize, Serialize};

// type Record = (String, String, u64);

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    email: String,
    age: u64,
}

pub fn csv_file_reader() -> Result<(), Box<dyn Error>> {
    // path to the csv file
    let file_path = "data/csv_data.csv";

    // this line opens the csv file from the path passed into it
    let file = File::open(file_path)?;

    // this creates a new instance of ReaderBuilder struct allows you to customize how the csv data is read
    // .has_headers checks if the .csv file has headers and treats the first row as it
    // .from_reader finalizes the ocnfiguration, creates a Reader instance which is used to read & parse the csv file.
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // this line iterates over the records in the csv file
    for results in rdr.records() {
        let record = results?;
        // this line checks if the record has a 3 fields and if it's an integer
        let age = record[2].parse::<i32>()?;

        // this line checks if the age is greater than 25 and then filters the record
        if age > 25 {
            println!("This is the record {:?}", record);
        }

    }

    Ok(())
}

pub fn csv_file_writer() -> Result<(), Box<dyn Error>> {
     // path to the csv file
    let file_path = "data/testing.csv";

    // this line opens the csv file from the path passed into it
    // let file = File::open(file_path)?;

    let file = File::create(file_path)?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["name", "email", "age"])?;
    writer.write_record(&["martin", "martin@gmail.edu", "32"])?;

    
    writer.flush()?;
    
    println!("CSV data has been written successfully.");

    let read_file_path = "data/testing.csv";

    let file_read = File::open(read_file_path)?;

    let mut rder = ReaderBuilder::new().has_headers(true).from_reader(file_read);

    for res in rder.records() {

        println!("This is the {:?}", res);
    }

    Ok(())
}


pub fn csv_file_reader_using_serde() -> Result<(), Box<dyn Error>> {
    // path to the csv file
    let file_path = "data/csv_data.csv";

    // this line opens the csv file from the path passed into it
    let file = File::open(file_path)?;

    // this creates a new instance of ReaderBuilder struct allows you to customize how the csv data is read
    // .has_headers checks if the .csv file has headers and treats the first row as it
    // .from_reader finalizes the ocnfiguration, creates a Reader instance which is used to read & parse the csv file.
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // this line iterates over the records in the csv file
    for results in rdr.deserialize() {
        let record: Record = results?;
        println!("This is the record {:#?}", record);

    }

    Ok(())
}
