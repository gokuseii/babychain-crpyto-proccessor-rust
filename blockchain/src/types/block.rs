use keccak_rust::Keccak;

use crate::traits::Hashable;
use crate::types::transaction::Transaction;
use crate::types::Hash;

#[derive(Clone, Debug)]
pub struct Block {
    pub id: Option<Hash>,
    pub prev: Option<Hash>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(prev: Option<Hash>, transactions: Vec<Transaction>) -> Self {
        let mut block = Block {
            id: None,
            prev,
            transactions,
        };
        block.update_hash();
        block
    }

    fn update_hash(&mut self) {
        self.id = Some(self.hash());
    }
}

impl Hashable for Block {
    fn hash(&self) -> Hash {
        let mut keccak = Keccak::new(256);
        if self.prev.is_some() {
            keccak.update(&self.prev.as_ref().unwrap());
        }
        for transaction in &self.transactions {
            keccak.update(&transaction.hash());
        }
        keccak.hash()
    }
}
