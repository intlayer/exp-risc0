#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

use polars::prelude::*;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let df = DataFrame::default();
    assert!(df.is_empty());
    println!("{}", df);

    // write public output to the journal
    env::commit(&input);
}
