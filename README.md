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

<blockquote
 class="twitter-tweet"><p lang="en" 
dir="ltr">cont.<br><br>wrote a bot that claims and 
transfers the <a 
href="https://twitter.com/search?q=%24ARB&amp;src=ctag&amp;ref_src=twsrc%5Etfw">$ARB</a>
 airdrop<br><br>it claimed 190 addresses in 0.16 seconds on a
 local node. rust concurrency is so much better than anything else out 
there <a 
href="https://t.co/xRz7RB0imy">https://t.co/xRz7RB0imy</a> 
<a 
href="https://t.co/2mtaE131eV">pic.twitter.com/2mtaE131eV</a></p>&mdash;
 makemake ⛈️ (@makemake_kbo) <a 
href="https://twitter.com/makemake_kbo/status/1638184776329732098?ref_src=twsrc%5Etfw">March
 21, 2023</a></blockquote>

## Disclaimer

This was tested and *should* work, see the licence for a warranty disclaimer(tldr; there is no warranty).  

