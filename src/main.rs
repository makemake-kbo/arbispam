mod errors;
mod read_csv;

use crate::errors::errors::print_help;
use std::env;

use ethers::prelude::*;

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 5 {
        let provider = Provider::<Http>::try_from(&args[3])?;

    } else {
        print_help();
        panic!("No command line arguments provided or too many args!");
    }
    Ok(())
}