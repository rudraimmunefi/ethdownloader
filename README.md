### Ethdownloader

Simple program to download source code from Etherscan API

## Description
Ethdownloader is a tool that allows you to download smart contract source code using the Etherscan API.

## Setup
1. Get an API key from [Etherscan](https://etherscan.io/apis)
2. Set your API key as an environment variable:
```bash
export ETHERSCAN_KEY=your_api_key
```

## Usage
Run the program with required contract address:
```bash
cargo run -- -c 0xContractAddress
```

You can also provide the API key directly via command line:
```bash
cargo run -- -k your_api_key -c 0xContractAddress
```

