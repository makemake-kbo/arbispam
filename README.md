# Arbispam - Rust program to mass claim the arbitrum airdrop

## Setup

Compile it with cargo or use cargo run.
```
ARBISPAM
Command line arguments _*required*_:
1) Path to CSV list of SKs
2) Claim address
3) Token address
4) HTTP RPC
5) Address to recieve
6) Chain ID
```
Example:

```bash
./arbispam ../../test.csv 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 0x5FbDB2315678afecb367f032d93F642f64180aa3 http://127.0.0.1:8545/ 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 31337
```

## Benchmarks

<img width="835" alt="Screenshot 2023-03-23 at 20 31 02" src="https://user-images.githubusercontent.com/55022497/227328952-19195c6c-b385-4939-8dc0-72c6a7a02497.png">

## Disclaimer

This was tested and *should* work, see the licence for a warranty disclaimer(tldr; there is no warranty).  

