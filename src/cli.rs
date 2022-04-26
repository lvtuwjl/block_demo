use super::*;
use crate::blockchain::*;
use crate::transaction::*;
use crate::wallets::*;
use clap::{App, Arg};
use std::process::exit;

pub struct Cli {
    // bc: Blockchain,
}

impl Cli {
    pub fn new() -> Cli {
        // Ok(Cli {
        //     bc: Blockchain::new()?,
        // })
        Cli {}
    }

    pub fn run(&mut self) -> Result<()> {
        info!("run app");
        let matches = App::new("block_demo")
            .version("0.1")
            .author("jianli. newyearwjl@gamil.com")
            .about("reimplement blockchain_go in rust: a simple blockchian for learning")
            .subcommand(App::new("printchain").about("print all the chain blocks"))
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
                    .arg(Arg::from_usage("<amount> 'Amount to send'")),
            )
            .get_matches();

        // print debugger log
        // println!("matches: {:?}", matches);

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.value_of("address") {
                let address = address.as_bytes();
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(address);

                let mut balance = 0;
                for out in utxos {
                    balance += out.value
                }
                println!("Balance: {}\n", balance);
                // self.addblock(String::from(c))?;
            }
        }

        if let Some(_) = matches.subcommand_matches("createwallet") {
            // self.print_chain();
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            // let bc = Blockchain::new()?;
            // for b in bc.iter() {
            //     println!("block: {:#?}", b);
            // }
            println!["success: address {}", address];
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            let bc = Blockchain::new()?;
            for b in bc.iter() {
                println!("block: {:#?}", b);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("creatblockchain") {
            if let Some(address) = matches.value_of("address") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
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
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;

            bc.mine_block(vec![tx])?;
            println!("success!");
        }

        Ok(())
    }

    // fn print_chain(&mut self) {
    //     for b in &mut self.bc {
    //         println!("block: {:#?}", b);
    //     }
    // }

    // fn addblock(&mut self, data: String) -> Result<()> {
    //     self.bc.add_block(data)
    // }
}
