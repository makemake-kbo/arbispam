mod transaction;
mod errors;
mod read_csv;

use crate::errors::print_help;
use crate::read_csv::read_csv_from_path;
use crate::transaction::*;

use std::env;

use ethers::prelude::*;

fn string_to_str(s: &String) -> &'static str {
    Box::leak(s.clone().into_boxed_str())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 7 {
        let claim_contract: &str = string_to_str(&args[2]);
        let token_contract: &str = string_to_str(&args[3]);
        let receiver_address: &str = string_to_str(&args[5]);
        let chain_id: u64 = args[6].parse::<u64>().unwrap();

        // Get provider
        let provider = Provider::<Http>::try_from(&args[4])?;

        // Vec<String> of private keys loaded from csv
        let private_keys = read_csv_from_path(&args[1]);
        println!("Private keys loaded: {:?}", &private_keys.len());

        let mut handles = Vec::new();

        // Iterate over private keys and send claim and transfer transactions
        for sk in private_keys.into_iter() {
            let provider_clone = provider.clone();
            let wallet: LocalWallet = sk.parse::<LocalWallet>()?.with_chain_id(chain_id.clone());
            let claim_contract_clone = claim_contract.clone();
            let token_contract_clone = token_contract.clone();
            let receiver_address_clone = receiver_address.clone();

            // Spawn a task for each wallet
            let handle = tokio::spawn(async move {
                send_claim_transaction(wallet.clone(), provider_clone.clone(), claim_contract_clone).await.ok();
                send_transfer_transaction(wallet, provider_clone, token_contract_clone, receiver_address_clone).await.ok();
            });
            // Add task to handles
            handles.push(handle);
        }
        futures::future::join_all(handles).await;

        Ok(())
    } else {
        print_help();
        panic!("No command line arguments provided or too many args!");
    }
}