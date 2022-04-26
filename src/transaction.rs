use super::*;
use crate::utxoset::*;
use crate::wallets::*;
use bincode::serialize;
use bitcoincash_addr::Address;
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::sha2::Sha256;
use failure::format_err;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SUBSIDY: i32 = 10;

/// TXInput represents a transaction input
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
    // pub script_sig: String,
}

/// TXOutput represents a transaction output
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>,
}

/// Transaction represents a Bitcoin transaction
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {
    /// NewUTXOTransaction creates a new transaction
    pub fn new_UTXO(from: &str, to: &str, amount: i32, utxo: &UTXOSet) -> Result<Transaction> {
        info!("new UTXO Transaction from: {} to: {}", from, to);
        let mut vin = Vec::new();

        let wallets = Wallets::new()?;

        // let e = Err(format_err!("wallets not found"));
        let wallet = match wallets.get_wallet(from) {
            Some(w) => w,
            None => return Err(format_err!("from wallet not found")),
        };

        if let None = wallets.get_wallet(&to) {
            return Err(format_err!("to wallet not found"));
        };

        let mut pub_key_hash = wallet.public_key.clone();
        hash_pub_key(&mut pub_key_hash);
        // let /
        let acc_v = utxo.find_spendable_outputs(&pub_key_hash, amount)?;

        if acc_v.0 < amount {
            error!("Not Enough balance");
            return Err(format_err!(
                "Not Enough balance: current balance {}",
                acc_v.0
            ));
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let input = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    // script_sig: String::from(from),
                    signature: Vec::new(),
                    pub_key: wallet.public_key.clone(),
                };
                vin.push(input);
            }
        }
        let mut vout = vec![TXOutput::new(amount, to.to_string())?];

        if acc_v.0 > amount {
            vout.push(TXOutput::new(acc_v.0 - amount, from.to_string())?)
        }

        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };

        // tx.set_id()?;
        tx.id = tx.hash()?;
        utxo.blockchain
            .sign_transaction(&mut tx, &wallet.secret_key)?;
        Ok(tx)
    }

    /// NewCoinbaseTX creates a new coinase transaction
    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        info!("new coinbase Transaction to: {}", to);
        if data.is_empty() {
            data = format!("Reward to '{}'", to);
        }
        let wallets = Wallets::new()?;
        if let None = wallets.get_wallet(&to) {
            return Err(format_err!("Coinbase wallet not found"));
        };

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TXInput {
                txid: String::new(),
                vout: -1,
                signature: Vec::new(),
                pub_key: Vec::from(data.as_bytes()),
            }],
            vout: vec![TXOutput::new(SUBSIDY, to)?],
        };

        tx.id = tx.hash()?;
        Ok(tx)

        // if data == String::from("") {
        //     data += &format!("Reward to '{}'", to);
        // }

        // let mut tx = Transaction {
        //     id: String::new(),
        //     vin: vec![TXInput {
        //         txid: String::new(),
        //         vout: -1,
        //         // script_sig: data,
        //         signature: String::new(),
        //         pub_key: Vec::from(data.as_bytes()),
        //     }],
        //     vout: vec![TXOutput::new(SUBSIDY, to)?],
        // };
        // tx.set_id()?;
        // Ok(tx)
    }

    // fn set_id(&mut self) -> Result<()> {
    //     let mut hasher = Sha256::new();
    //     let data = serialize(self)?;
    //     hasher.input(&data);

    //     self.id = hasher.result_str();
    //     Ok(())
    // }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }

    pub fn verify(&mut self, prev_TXs: HashMap<String, Transaction>) -> Result<bool> {
        if self.is_coinbase() {
            return Ok(true);
        }

        for vin in &self.vin {
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.trim_copy();
        for in_id in 0..self.vin.len() {
            let prev_Tx = prev_TXs.get(&self.vin[in_id].txid).unwrap();
            tx_copy.vin[in_id].signature.clear();
            tx_copy.vin[in_id].pub_key = prev_Tx.vout[self.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash()?;
            tx_copy.vin[in_id].pub_key = Vec::new();

            if !ed25519::verify(
                &tx_copy.id.as_bytes(),
                &self.vin[in_id].pub_key,
                &self.vin[in_id].signature,
            ) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn sign(
        &mut self,
        private_key: &[u8],
        prev_TXs: HashMap<String, Transaction>,
    ) -> Result<()> {
        if self.is_coinbase() {
            return Ok(());
        }

        for vin in &self.vin {
            if prev_TXs.get(&vin.txid).unwrap().id.is_empty() {
                return Err(format_err!("ERROR: Previous transaction is not correct"));
            }
        }

        let mut tx_copy = self.trim_copy();
        for in_id in 0..tx_copy.vin.len() {
            let prev_Tx = prev_TXs.get(&tx_copy.vin[in_id].txid).unwrap();
            tx_copy.vin[in_id].signature.clear();
            tx_copy.vin[in_id].pub_key = prev_Tx.vout[tx_copy.vin[in_id].vout as usize]
                .pub_key_hash
                .clone();
            tx_copy.id = tx_copy.hash()?;
            tx_copy.vin[in_id].pub_key = Vec::new();
            let signature = ed25519::signature(tx_copy.id.as_bytes(), private_key);
            self.vin[in_id].signature = signature.to_vec();
        }
        Ok(())
    }

    pub fn hash(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.id = String::new();
        let data = serialize(&copy)?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        Ok(hasher.result_str())
    }

    fn trim_copy(&self) -> Transaction {
        let mut vin = Vec::new();
        let mut vout = Vec::new();

        for v in &self.vin {
            vin.push(TXInput {
                txid: v.txid.clone(),
                vout: v.vout.clone(),
                signature: Vec::new(),
                pub_key: Vec::new(),
            })
        }

        for v in &self.vout {
            vout.push(TXOutput {
                value: v.value,
                pub_key_hash: v.pub_key_hash.clone(),
            })
        }

        Transaction {
            id: self.id.clone(),
            vin,
            vout,
        }
    }
}

// impl TXInput {
//     pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
//         let mut pubkeyhash = self.pub_key.clone();
//         hash_pub_key(&mut pubkeyhash);
//         pubkeyhash == pub_key_hash
//     }
//     // pub fn can_unlock_output_with(&self, unlockingData: &str) -> bool {
//     //     self.script_sig == unlockingData
//     // }
// }

impl TXOutput {
    // pub fn can_be_unlock_with(&self, unlockingData: &str) -> bool {
    //     self.script_pub_key == unlockingData
    // }
    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash == pub_key_hash
    }

    fn lock(&mut self, address: &str) -> Result<()> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock: {}", address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    pub fn new(value: i32, address: String) -> Result<Self> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        txo.lock(&address)?;
        Ok(txo)
    }
}
