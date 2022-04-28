#![allow(dead_code, non_snake_case)]

mod block;
mod blockchain;
mod cli;
mod server;
mod transaction;
mod utxoset;
mod wallets;

#[macro_use]
extern crate log;

pub type Result<T> = std::result::Result<T, failure::Error>;

// use std::fmt::Error;

// use std::pin::Pin;
// use blockchain::*;
// use std::thread::sleep;
// use std::time::Duration;
use crate::cli::Cli;

use env_logger::Env;
fn main() {
    // println!("Hello, world!");
    // let mut bc = Blockchain::new();
    // sleep(Duration::from_millis(10));
    // bc.add_block(String::from("Send 1 BTC to Ivan"))?;
    // sleep(Duration::from_millis(30));
    // bc.add_block(String::from("Send 2 more BTC to Ivan"))?;

    // println!("Blockchain: {:#?}", bc);
    // Ok(())
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    let mut cli = Cli::new();
    // cli.run()?;
    if let Err(e) = cli.run() {
        println!("Error: {}", e);
    }

    // Ok(())

    // add println
    println!("{}", "test println");
}
