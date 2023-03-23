use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Reads CSV ant outputs a Vec<string> of SKs
// Takes in path to CSV as an argument
pub fn read_csv_from_path(csv_file: &str) -> Vec<String> {
    let file = File::open(csv_file).expect("Error opening file.");
    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        // Split line by comma and push to data vector
        let values: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        for val in values {
            data.push(val);
        }
    }
    data
}
