mod transaction;
mod errors;
mod read_csv;


use crate::read_csv::read_csv_from_path;
use crate::transaction::*;
use crate::errors::errors::print_help;
use std::env;

use ethers::prelude::*;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

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
            let wallet: LocalWallet = sk.parse::<LocalWallet>()?;
            let claim_contract_clone = string_to_static_str(claim_contract.clone());
            let token_contract_clone = string_to_static_str(token_contract.clone());
            let receiver_address_clone = string_to_static_str(receiver_address.clone());

            let handle = tokio::spawn(async move {
                send_claim_transaction(wallet.clone(), provider_clone.clone(), claim_contract_clone.clone()).await.ok();
                send_transfer_transaction(wallet, provider_clone, token_contract_clone, receiver_address_clone).await.ok();
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