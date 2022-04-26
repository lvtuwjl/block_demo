use super::*;
use crate::block::*;
use crate::transaction::*;
use bincode::{deserialize, serialize};
use failure::format_err;
use sled;
use std::collections::HashMap;

const GENESIS_COINBASE_DATA: &str =
    "The times 03/Jan/2009 Chancellor on brink of second bailout for banks";

/// Bockchain keeps a sequence of Blocks
#[derive(Debug)]
pub struct Blockchain {
    // blocks: Vec<Block>,
    pub tip: String,
    // current_hash: String,
    pub db: sled::Db,
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
    }

    /// CreateBlockchain creates a new blockchain DB
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");
        std::fs::remove_dir_all("data/blocks")?;
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
    pub fn mine_block(&mut self, mut transactions: Vec<Transaction>) -> Result<Block> {
        info!("mine a new block");
        for tx in &mut transactions {
            if !self.verify_transaction(tx)? {
                return Err(format_err!("ERROR: Invalid transaction"));
            }
        }

        let lasthash = self.db.get("LAST")?.unwrap();

        let newblock = Block::new_block(transactions, String::from_utf8(lasthash.to_vec())?)?;

        self.db.insert(newblock.get_hash(), serialize(&newblock)?)?;
        self.db.insert("LAST", newblock.get_hash().as_bytes())?;
        self.db.flush()?;

        self.tip = newblock.get_hash();
        Ok(newblock)
    }

    /// Iterator returns a BlockchainIterator
    pub fn iter(&self) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: self.tip.clone(),
            bc: &self,
        }
    }

    /// FindUTXO finds and returns all unspent transaction outputs
    pub fn find_UTXO(&self) -> HashMap<String, TXOutputs> {
        let mut utxos = HashMap::<String, TXOutputs>::new();
        let mut spend_txos = HashMap::<String, Vec<i32>>::new();

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

        utxos
    }

    pub fn find_transaction(&self, id: &str) -> Result<Transaction> {
        for b in self.iter() {
            for tx in b.get_transaction() {
                if tx.id == id {
                    return Ok(tx.clone());
                }
            }
        }

        return Err(format_err!("Transaction is not found"));
    }

    fn get_prev_TXs(&self, tx: &Transaction) -> Result<HashMap<String, Transaction>> {
        let mut prev_TXs = HashMap::new();
        for vin in &tx.vin {
            let prev_TX = self.find_transaction(&vin.txid)?;
            prev_TXs.insert(prev_TX.id.clone(), prev_TX);
        }

        Ok(prev_TXs)
    }

    pub fn sign_transaction(&self, tx: &mut Transaction, private_key: &[u8]) -> Result<()> {
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.sign(private_key, prev_TXs)?;
        Ok(())
    }

    pub fn verify_transaction(&self, tx: &mut Transaction) -> Result<bool> {
        if tx.is_coinbase() {
            return Ok(true);
        }
        let prev_TXs = self.get_prev_TXs(tx)?;
        tx.verify(prev_TXs)
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
