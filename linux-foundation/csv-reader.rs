use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Record {
    student: String,
    age: u8,
    GPA: f32,
}

impl Record {
    fn from_csv_line(line: &str) -> Result<Record, Box<dyn Error>> {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 3 {
            return Err("Invalid CSV line".into());
        }

        let student = fields[0].to_string();
        let age = fields[1].parse::<u8>()?;
        let GPA = fields[2].parse::<f32>()?;

        Ok(Record { student, age, GPA })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "data.csv";
    let mut records: Vec<Record> = Vec::new();

    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let record = Record::from_csv_line(&line)?;
        records.push(record);
    }

    let total_score: f32 = records.iter().map(|r| r.GPA).sum();
    let average_score = total_score / records.len() as f32;

    println!("Number of students: {}", records.len());
    println!("Average GPA: {:.2}", average_score);

    Ok(())
}
