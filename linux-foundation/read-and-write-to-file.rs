use std::fs::File;
use std::io::{self, BufRead, BufReader, Write}; 

fn main() -> io::Result<()> {
    let content = File::open("input.txt")?;
    let file_buffer = BufReader::new(content);

    for line in file_buffer.lines() {
        let input = line?;
        let num: i8 = input.trim().parse().expect("Invalid number in file"); 

        let result = match num {
            -1 => "is negative!".into(),
            0 => "is zero!".into(),
            1 => "is positive".into(),
            _ => Err(()),
        }
        println!("Input `{}: {}", number, result); 

        let mut output_file = File::create("output.txt")?; 

        writeln!(output_file, "{}", result)?;
    } 

    Ok(())
} 
