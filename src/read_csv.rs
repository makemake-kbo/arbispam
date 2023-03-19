use std::fs::File;
use std::io::{self, BufRead};
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
#[allow(dead_code)]
pub fn read_csv(path: &str) -> Vec<String> {
    let mut sks: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(sk) = line {
                sks.push(sk);
            }
        }
    }
    return sks;
}
