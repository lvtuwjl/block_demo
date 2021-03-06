use super::*;
use crate::blockchain::*;
use crate::server::Server;
use crate::transaction::*;
use crate::utxoset::*;
use crate::wallets::*;
use bitcoincash_addr::Address;
use clap::{App, Arg};
use std::process::exit;

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }

    pub fn run(&mut self) -> Result<()> {
        info!("run app");
        let matches = App::new("block_demo")
            .version("0.1")
            .author("jianli. newyearwjl@gamil.com")
            .about("reimplement blockchain_go in rust: a simple blockchian for learning")
            .subcommand(App::new("printchain").about("print all the chain blocks"))
            .subcommand(App::new("createwallet").about("create a wallet"))
            .subcommand(App::new("listaddresses").about("list all addresses"))
            .subcommand(App::new("reindex").about("reindex UTXO"))
            .subcommand(
                App::new("startnode")
                    .about("start the node server")
                    .arg(Arg::from_usage("<port> 'the port server bind to locally'")),
            )
            .subcommand(
                App::new("startminer")
                    .about("start the miner server")
                    .arg(Arg::from_usage("<port> 'the port server bind to locally'"))
                    .arg(Arg::from_usage("<address> 'wallet address'")),
            )
            .subcommand(
                App::new("getbalance")
                    .about("get balance in the blockchain")
                    .arg(Arg::from_usage(
                        "<address> 'The address to get balance for'",
                    )),
            )
            .subcommand(App::new("createblockchain").about("create blokchain").arg(
                Arg::from_usage("<address> 'The address to send genesis block reward to'"),
            ))
            .subcommand(
                App::new("send")
                    .about("send in the blockchain")
                    .arg(Arg::from_usage("<from> 'Source wallet address'"))
                    .arg(Arg::from_usage("<to> 'Destination wallet address'"))
                    .arg(Arg::from_usage("<amount> 'Amount to send'"))
                    .arg(Arg::from_usage(
                        "-m --mine 'the from address mine immediately'",
                    )),
            )
            .get_matches();

        // print debugger log
        // println!("matches: {:?}", matches);

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.value_of("address") {
                // let pub_key_hash = Address::decode(address).unwrap().body;
                // let bc = Blockchain::new()?;
                // let utxo_set = UTXOSet { blockchain: bc };
                // let utxos = utxo_set.find_UTXO(&pub_key_hash)?;

                let balance = cmd_get_balance(address)?;
                println!("Balance: {}\n", balance);
                // for out in utxos.outputs {
                //     balance += out.value;
                // }
                // println!("Balance: {}\n", balance);
            }
        } else if let Some(_) = matches.subcommand_matches("createwallet") {
            println!("address: {}", cmd_create_wallet()?);
        } else if let Some(_) = matches.subcommand_matches("printchain") {
            cmd_print_chain()?;
        } else if let Some(_) = matches.subcommand_matches("reindex") {
            let count = cmd_reindex()?;
            println!("Done! There are {} transactions in the UTXO set.", count);
        } else if let Some(_) = matches.subcommand_matches("listaddresses") {
            cmd_list_address()?;
        } else if let Some(ref matches) = matches.subcommand_matches("createblockchain") {
            if let Some(address) = matches.value_of("address") {
                cmd_create_blockchain(address)?;
            }
        } else if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.value_of("from") {
                address
            } else {
                println!("from not supply!: usage\n{}", matches.usage());
                exit(1)
            };

            let to = if let Some(address) = matches.value_of("to") {
                address
            } else {
                println!("to not supply!: usage\n{}", matches.usage());
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.value_of("amount") {
                amount.parse()?
            } else {
                println!("amount in send not supply!: usage\n{}", matches.usage());
                exit(1);
            };

            if matches.is_present("mine") {
                cmd_send(from, to, amount, true)?
            } else {
                cmd_send(from, to, amount, false)?;
            }
        } else if let Some(ref matches) = matches.subcommand_matches("startnode") {
            println!("11 {:?}",matches.value_of("port"));
            if let Some(port) = matches.value_of("port") {
                println!("Start node...");
                let bc = Blockchain::new()?;
                let utxo_set = UTXOSet { blockchain: bc };
                let server = Server::new(port, "", utxo_set)?;
                server.start_server()?;
            }
        } else if let Some(ref matches) = matches.subcommand_matches("startminer") {
            let address = if let Some(address) = matches.value_of("address") {
                address
            } else {
                println!("address not supply!: usage\n{}", matches.usage());
                exit(1)
            };

            let port = if let Some(port) = matches.value_of("port") {
                port
            } else {
                println!("port not supply!: usage\n{}", matches.usage());
                exit(1)
            };

            println!("Start miner node...");
            let bc = Blockchain::new()?;
            let utxo_set = UTXOSet { blockchain: bc };
            let server = Server::new(port, address, utxo_set)?;
            server.start_server()?;
        }

        // if let Some(_) = matches.subcommand_matches("createwallet") {
        //     // self.print_chain();
        //     let mut ws = Wallets::new()?;
        //     let address = ws.create_wallet();
        //     ws.save_all()?;
        //     // let bc = Blockchain::new()?;
        //     // for b in bc.iter() {
        //     //     println!("block: {:#?}", b);
        //     // }
        //     println!["success: address {}", address];
        // }

        // if let Some(_) = matches.subcommand_matches("printchain") {
        //     let bc = Blockchain::new()?;
        //     for b in bc.iter() {
        //         println!("block: {:#?}", b);
        //     }
        // }

        // if let Some(_) = matches.subcommand_matches("reindex") {
        //     let bc = Blockchain::new()?;
        //     let utxo_set = UTXOSet { blockchain: bc };
        //     utxo_set.reindex()?;
        //     let count = utxo_set.count_transactions()?;
        //     println!("Done! There are {} transactions in the UTXO set.", count);
        // }

        // if let Some(_) = matches.subcommand_matches("listaddresses") {
        //     let ws = Wallets::new()?;
        //     let addresses = ws.get_all_addresses();
        //     println!("addresses: ");
        //     for ad in addresses {
        //         println!("{}", ad);
        //     }
        // }

        // if let Some(ref matches) = matches.subcommand_matches("creatblockchain") {
        //     if let Some(address) = matches.value_of("address") {
        //         let address = String::from(address);
        //         let bc = Blockchain::create_blockchain(address)?;

        //         let utxo_set = UTXOSet { blockchain: bc };
        //         utxo_set.reindex()?;
        //         println!("create blockchain");
        //     }
        // }

        // if let Some(ref matches) = matches.subcommand_matches("send") {
        //     let from = if let Some(address) = matches.value_of("from") {
        //         address
        //     } else {
        //         println!("from not supply!: usage\n{}", matches.usage());
        //         exit(1)
        //     };

        //     let to = if let Some(address) = matches.value_of("to") {
        //         address
        //     } else {
        //         println!("to not supply!: usage\n{}", matches.usage());
        //         exit(1)
        //     };

        //     let amount: i32 = if let Some(amount) = matches.value_of("amount") {
        //         amount.parse()?
        //     } else {
        //         println!("amount in send not supply!: usage\n{}", matches.usage());
        //         exit(1)
        //     };

        //     let bc = Blockchain::new()?;
        //     let mut utxo_set = UTXOSet { blockchain: bc };
        //     let tx = Transaction::new_UTXO(from, to, amount, &utxo_set)?;
        //     let cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;
        //     let new_block = utxo_set.blockchain.mine_block(vec![cbtx, tx])?;

        //     utxo_set.update(&new_block)?;
        //     println!("success!");
        // }

        Ok(())
    }
}

fn cmd_send(from: &str, to: &str, amount: i32, mine_now: bool) -> Result<()> {
    let bc = Blockchain::new()?;
    let mut utxo_set = UTXOSet { blockchain: bc };
    let wallets = Wallets::new()?;
    let wallet = wallets.get_wallet(from).unwrap();
    let tx = Transaction::new_UTXO(wallet, to, amount, &utxo_set)?;
    if mine_now {
        let cbtx = Transaction::new_coinbase(from.to_string(), String::from("reward!"))?;
        let new_block = utxo_set.blockchain.mine_block(vec![cbtx, tx])?;

        utxo_set.update(&new_block)?;
    } else {
        Server::send_transaction(&tx, utxo_set)?;
    }

    println!("success");
    Ok(())
}

fn cmd_create_wallet() -> Result<String> {
    let mut ws = Wallets::new()?;
    let address = ws.create_wallet();
    ws.save_all()?;
    Ok(address)
}

fn cmd_reindex() -> Result<i32> {
    let bc = Blockchain::new()?;
    let utxo_set = UTXOSet { blockchain: bc };

    utxo_set.reindex()?;
    utxo_set.count_transactions()
}

fn cmd_create_blockchain(address: &str) -> Result<()> {
    let address = String::from(address);
    let bc = Blockchain::create_blockchain(address)?;

    let utxo_set = UTXOSet { blockchain: bc };
    utxo_set.reindex()?;
    println!("create blockchain");
    Ok(())
}

fn cmd_get_balance(address: &str) -> Result<i32> {
    let pub_key_hash = Address::decode(address).unwrap().body;
    let bc = Blockchain::new()?;
    let utxo_set = UTXOSet { blockchain: bc };
    let utxos = utxo_set.find_UTXO(&pub_key_hash)?;

    let mut balance = 0;
    for out in utxos.outputs {
        balance += out.value;
    }

    Ok(balance)
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }

    Ok(())
}

fn cmd_list_address() -> Result<()> {
    let ws = Wallets::new()?;
    let addresses = ws.get_all_addresses();
    println!("addresses: ");
    for ad in addresses {
        println!("{}", ad);
    }
    Ok(())
}
