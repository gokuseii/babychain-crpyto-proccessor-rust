use keccak_rust::Keccak;

use crate::traits::Hashable;
use crate::types::operation::Operation;
use crate::types::Hash;

struct Transaction {
    id: Hash,
    operations: Vec<Operation>,
    nonce: u64,
}

impl Transaction {
    fn new(nonce: u64, operations: Vec<Operation>) -> Transaction {
        let mut keccak = Keccak::new(256);
        let data = format!("{:?}", (operations.clone(), nonce));
        keccak.update(&data);
        Transaction {
            id: keccak.hash(),
            operations,
            nonce,
        }
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
