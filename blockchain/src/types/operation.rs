use keccak_rust::Keccak;

use crate::traits::Hashable;
use crate::types::{Account, Balance, Hash, Signature};

#[derive(Debug, Clone)]
pub struct Operation {
    sender: Account,
    receiver: Account,
    amount: Balance,
    signature: Signature,
}

impl Operation {
    pub fn new(
        sender: Account,
        receiver: Account,
        amount: Balance,
        index_wallet: Option<usize>,
    ) -> Self {
        let data = format!("{:?}", (sender.clone().id, receiver.clone().id, amount));
        let signature = sender.sign(&data, index_wallet);
        Operation {
            sender,
            receiver,
            amount,
            signature,
        }
    }

    pub fn sender(&self) -> Account {
        self.sender.clone()
    }

    pub fn receiver(&self) -> Account {
        self.receiver.clone()
    }

    pub fn amount(&self) -> Balance {
        self.amount
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    fn data(&self) -> String {
        format!(
            "{:?}",
            (
                self.sender.clone().id,
                self.receiver.clone().id,
                self.amount
            )
        )
    }

    pub fn verify(&self, index_wallet: Option<usize>) -> bool {
        let data = self.data();
        if self.sender.balance() < self.amount
            && self.sender.verify(&data, &self.signature, index_wallet)
        {
            return false;
        }
        true
    }
}

impl Hashable for Operation {
    fn hash(&self) -> Hash {
        let mut keccak = Keccak::new(256);
        let data = self.data();
        keccak.update(&data);
        keccak.hash()
    }
}
