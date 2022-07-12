use keccak_rust::Keccak;

use crate::traits::Hashable;
use crate::types::blockhain::Blockchain;
use crate::types::operation::Operation;
use crate::types::{Error, Hash};

#[derive(Clone, Debug)]
pub struct Transaction {
    id: Hash,
    pub operations: Vec<Operation>,
    nonce: u64,
}

impl Transaction {
    pub fn new(nonce: u64, operations: Vec<Operation>) -> Transaction {
        let mut keccak = Keccak::new(256);
        let data = format!("{:?}", (operations.clone(), nonce));
        keccak.update(&data);
        Transaction {
            id: keccak.hash(),
            operations,
            nonce,
        }
    }

    pub fn validate(&self, chain: &mut Blockchain) -> Result<(), Error> {
        let hash = self.hash();
        let mut hashes_operation: Vec<Hash> = Vec::new();

        for chain_tx in &chain.tx_database {
            if hash == chain_tx.hash() {
                return Err("Duplicate transactions".to_string());
            }
            for operation in &chain_tx.operations {
                hashes_operation.push(operation.hash());
            }
        }

        for operation in &self.operations {
            if hashes_operation.contains(&operation.hash()) {
                return Err("Duplicate operation".to_string());
            }

            let amount = operation.amount();
            if amount <= 0 {
                return Err("Amount operation must be more than zero".to_string());
            }

            match chain.coin_database.get(operation.sender().id.as_str()) {
                Some(balance) => {
                    if balance.clone() < operation.amount() {
                        return Err("Insufficient balance".to_string());
                    }
                }
                _ => return Err("Sender account not exist".to_string()),
            }

            chain.sub_from_balance(operation.sender().id, amount)?;
            if let Err(_) = chain.add_to_balance(operation.receiver().id, amount) {
                return Err("Overflow transaction amount".to_string());
            }
        }
        Ok(())
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> Hash {
        let mut keccak = Keccak::new(256);
        let data = format!("{:?}", (self.operations.clone(), self.nonce));
        keccak.update(&data);
        keccak.hash()
    }
}
