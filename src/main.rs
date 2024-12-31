use clap::{Arg, Command};
use ethers::etherscan::Client;
use ethers::prelude::*;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let matches = Command::new("etherscan-source")
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .help("Etherscan API key")
                .required(false),
        )
        .arg(
            clap::Arg::new("contract")
                .short('c')
                .long("contract")
                .help("Contract address")
                .required(true),
        )
        .get_matches();

    // Get API key from args or env
    let api_key = matches
        .get_one::<String>("key")
        .map(|s| s.to_string())
        .or_else(|| std::env::var("ETHERSCAN_KEY").ok())
        .expect("Must provide API key via -k flag or ETHERSCAN_KEY env var");

    // Get contract address from args
    let address = matches
        .get_one::<String>("contract")
        .expect("Contract address is required")
        .parse::<Address>()
        .expect("Invalid contract address");

    println!("Downloading source code...");

    // Initialize the client
    let client = Client::new(Chain::Mainnet, api_key).expect("Failed to create Etherscan client");

    let source = client.contract_source_code(address).await.unwrap();

    // Iterate over each Metadata item in the vector and print the proxy field
    for metadata in source.items {
        println!("{:?}", metadata.source_code);
    }
}
