use ethers::providers::{Http, Provider};
use ethers::prelude::*;
use std::convert::TryFrom;

abigen!(
    Claim,
    r#"[function claim() external]"#
);

abigen!(
    ERC20,
    r#"[function transfer(address to, uint256 amount) external returns (bool)]"#
);

// Tokio async function that takes in the provider and private key as a string and sends transaction to contract with a function claim with no arguments
#[tokio::main]
async fn send_claim_transaction(provider: Provider<Http>, sk: String, nonce: u32, _contract_address: String) -> Result<(), Box<dyn std::error::Error>> {
    let wallet = LocalWallet::new(&sk, Some(provider));

    // Address of the contract
    let contract_address = Address::from_str(_contract_address).unwrap();

    // ABI of the contract, if the abi points to an
    let contract_abi = include_bytes!("./abi/claim.json")?;

    // Create an instance of the contract using the provider and ABI
    let contract = Claim::new(contract_address, wallet.clone());

    // Call the `claim` function on the contract
    let tx = contract.claim().send().await?;

    println!("Claim transaction sent! Hash: {:?}", tx.hash);

    Ok(())
}

// Tokio async function that takes in the provider and private key as a string and sends transaction to contract to transfer claim
#[tokio::main]
async fn send_transfer_transaction(provider: Provider<Http>, sk: String, nonce: u32, _contract_address: String, receiver_address: String) -> Result<(), Box<dyn std::error::Error>> {
    let wallet = LocalWallet::new(&sk, Some(provider));

    // Address of the contract
    let contract_address = Address::from_str(_contract_address).unwrap();

    // ABI of the contract, if the abi points to an
    let contract_abi = include_bytes!("./abi/erc20.json")?;

    // Create an instance of the contract using the provider and ABI
    let contract = ERC20::new(contract_address, wallet.clone());

    // Get balance of address
    let balance = contract.balance_of().call(wallet.address()).await?;

    // Call the `claim` function on the contract
    let tx = contract.transfer(receiver_address.unwrap(), balance).send().await?;

    println!("Send transaction sent! Hash: {:?}", tx.hash);

    Ok(())
}
