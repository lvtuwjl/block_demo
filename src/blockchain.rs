use super::*;
use crate::block::*;
use crate::transaction::*;
use bincode::{deserialize, serialize};
use sled;
use std::collections::HashMap;

const GENESIS_COINBASE_DATA: &str =
    "The times 03/Jan/2009 Chancellor on brink of second bailout for banks";

/// Bockchain keeps a sequence of Blocks
#[derive(Debug)]
pub struct Blockchain {
    // blocks: Vec<Block>,
    tip: String,
    // current_hash: String,
    db: sled::Db,
}

/// BlockchainIterator is used to iterate over blockchain blocks
pub struct BlockchainIterator<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    /// NewBlockchian creates a new Blockchain db
    pub fn new() -> Result<Blockchain> {
        info!("open blockchain");
        let db = sled::open("data/blocks")?;

        let hash = db
            .get("LAST")?
            .expect("Must create a new block database forst");
        info!("Found block database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            tip: lasthash.clone(),
            db,
        })
        // match db.get("LAST")? {
        //     Some(hash) => {
        //         info!("Found block database");
        //         let lasthash = String::from_utf8(hash.to_vec())?;
        //         Ok(Blockchain {
        //             tip: lasthash.clone(),
        //             current_hash: lasthash,
        //             db,
        //         })
        //     }
        //     None => {
        //         info!("Creating new block database");
        //         let block = Block::new_genesis_bock();
        //         db.insert(block.get_hash(), serialize(&block)?)?;
        //         db.insert("LAST", block.get_hash().as_bytes())?;
        //         let bc = Blockchain {
        //             tip: block.get_hash(),
        //             current_hash: block.get_hash(),
        //             db,
        //         };
        //         bc.db.flush()?;
        //         Ok(bc)
        //     }
        // }
    }

    /// CreateBlockchain creates a new blockchain DB
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");
        let db = sled::open("/data/blocks")?;
        debug!("Creating new block database");

        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis: Block = Block::new_genesis_block(cbtx);

        db.insert(genesis.get_hash(), serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            tip: genesis.get_hash(),
            db,
        };

        bc.db.flush()?;
        Ok(bc)
    }

    /// MineBlock mines a new block with the provided transactions
    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<()> {
        info!("mine a new block");
        let lasthash = self.db.get("LAST")?.unwrap();

        let newblock = Block::new_block(transactions, String::from_utf8(lasthash.to_vec())?)?;

        self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
        self.db.insert("LAST", newblock.get_hash().as_bytes())?;
        self.db.flush()?;

        self.tip = newblock.get_hash();
        Ok(())
    }

    // / AddBlock saves provided data as a block in the blockchain
    // / Save the block into the database
    // pub fn add_block(&mut self, data: String) -> Result<()> {
    //     info!("add new block to the chain");
    //     let lasthash = self.db.get("LAST")?.unwrap();
    //     let newblock = Block::new_block(data, String::from_utf8(lasthash.to_vec())?)?;
    //     self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
    //     self.db.insert("LAST", newblock.get_hash().as_bytes())?;
    //     self.db.flush()?;

    //     self.tip = newblock.get_hash();
    //     self.current_hash = newblock.get_hash();

    //     Ok(())

    //     //     let prev = self.blocks.last().unwrap();
    //     //     // println!("hah {:?}",prev);
    //     //     let newblock = Block::new_block(data, prev.get_hash())?;
    //     //     self.blocks.push(newblock);
    //     //     Ok(())
    // }

    /// Iterator returns a BlockchainIterator
    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            bc: &self,
        }
    }

    /// FindUTXO finds and returns all unspent transaction outputs
    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {
        let mut utxos = HashMap::new();
        let mut spend_txos = HashMap::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spend_txos.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    match utxos.get_mut(&tx.id) {
                        Some(v) => {
                            v.outputs.push(tx.vout[index].clone());
                        }
                        None => {
                            utxos.insert(
                                tx.id.clone(),
                                TXOutputs {
                                    outputs: vec![tx.vout[index].clone()],
                                },
                            );
                        }
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        match spend_txos.get_mut(&i.txid) {
                            Some(v) => {
                                v.push(i.vout);
                            }
                            None => {
                                spend_txos.insert(i.txid.clone(), vec![i.vout]);
                            }
                        }
                    }
                }
            }
        }
        // for tx in unspend_TXs {
        //     for out in &tx.vout {
        //         if out.is_locked_with_key(pub_key_hash) {
        //             utxos.push(out.clone());
        //         }
        //     }
        // }

        utxos
    }

    /// FindUnspentTransactions returns a list of transactions containing unspent outputs
    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let unspend_TXs = self.find_unspent_transactions(pub_key_hash);

        for tx in unspend_TXs {
            for index in 0..tx.vout.len() {
                if tx.vout[index].is_locked_with_key(pub_key_hash) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }

                    accumulated += tx.vout[index].value;
                    if accumulated >= amount {
                        return (accumulated, unspent_outputs);
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }

    /// FindUnspentTransactions returns a list of transactions containing unspent outputs
    fn find_unspent_transactions(&self, pub_key_hash: &[u8]) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_TXs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.vout[index].is_locked_with_key(pub_key_hash) {
                        unspend_TXs.push(tx.to_owned())
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.uses_key(pub_key_hash) {
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }
        unspend_TXs
    }
}

impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}
