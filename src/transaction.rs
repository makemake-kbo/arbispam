use std::str::FromStr;
use ethers_contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::prelude::*;
use std::{sync::Arc};

// Generate bindings for the claim and erc20 token contracts
abigen!(
    Claim,
    r#"[function claim() external]"#
);

abigen!(
    ERC20,
    r#"[
        function transfer(address, uint256) external returns(bool)
        function balanceOf(address) external returns(uint256)
        ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

// Tokio async function that takes in the provider and private key as a string and sends transaction to contract with a function claim with no arguments
pub async fn send_claim_transaction(wallet: LocalWallet, provider: Provider<Http>, _contract_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    // Address of the contract
    let contract_address = Address::from_str(_contract_address).unwrap();

    // Create an instance of the contract using the provider and ABI
    let contract = Claim::new(contract_address.clone(), Arc::new(client));

    // Call the `claim` function on the contract
    contract.claim().send().await?;

    println!("Claim transaction sent!");

    Ok(())
}

// Tokio async function that takes in the provider and private key as a string and sends transaction to contract to transfer claim
pub async fn send_transfer_transaction(wallet: LocalWallet, provider: Provider<Http>, _contract_address: &str, receiver_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    // Address of the contract
    let contract_address = Address::from_str(_contract_address).unwrap();

    // Create an instance of the contract using the provider and ABI
    let contract = ERC20::new(contract_address.clone(), Arc::new(client));

    // Get balance of address
    let balance = contract.balance_of(wallet.address()).call().await?;

    // Convert receiver address to String
    let receiver_address = Address::from_str(receiver_address).unwrap();

    // Call the `claim` function on the contract
    contract.transfer(receiver_address, balance).send().await?;

    println!("Send transaction sent!");

    Ok(())
}
