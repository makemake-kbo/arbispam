mod transaction;
mod errors;
mod read_csv;


use crate::read_csv::read_csv_from_path;
use crate::transaction::*;
use crate::errors::errors::print_help;
use std::env;

use ethers::prelude::*;



#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 6 {
        let claim_contract: &String = &args[1];
        let token_contract: &String = &args[2];
        let receiver_address: &String = &args[4];

        // Get provider
        let provider = Provider::<Http>::try_from(&args[3])?;

        // Vec<String> of private keys loaded from csv
        let private_keys = read_csv_from_path(&args[1]);

        let mut handles = Vec::new();

        for sk in private_keys {
            let provider_clone = provider.clone();
            let sk_clone = sk.clone();
            let wallet: LocalWallet = sk.parse::<LocalWallet>()?;
            let handle = tokio::spawn(async move {

                send_claim_transaction(wallet, provider_clone, sk_clone, claim_contract).await;
                send_transfer_transaction(wallet, provider_clone, sk_clone, token_contract, receiver_address)
            });
            handles.push(handle);
        }

        futures::future::join_all(handles).await;

        Ok(())
    } else {
        print_help();
        panic!("No command line arguments provided or too many args!");
    }
}