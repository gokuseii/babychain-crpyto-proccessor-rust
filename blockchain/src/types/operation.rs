use keccak_rust::Keccak;

use crate::traits::Hashable;
use crate::types::{Account, AccountId, Balance, Hash, Signature};

#[derive(Debug)]
pub struct Operation {
    sender: Account,
    receiver_id: AccountId,
    amount: Balance,
    signature: Signature,
}

impl Operation {
    pub fn new(
        sender: Account,
        receiver_id: AccountId,
        amount: Balance,
        index_wallet: Option<usize>,
    ) -> Self {
        let data = format!("{:?}", (sender.clone().id, receiver_id.clone(), amount));
        let signature = sender.sign(&data, index_wallet);
        Operation {
            sender,
            receiver_id,
            amount,
            signature,
        }
    }

    pub fn amount(&self) -> Balance {
        self.amount
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn verify(&self, index_wallet: Option<usize>) -> bool {
        let data = format!(
            "{:?}",
            (
                self.sender.clone().id,
                self.receiver_id.clone(),
                self.amount
            )
        );
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
        let data = format!(
            "{:?}",
            (
                self.sender.clone().id,
                self.receiver_id.clone(),
                self.amount
            )
        );
        keccak.update(&data);
        keccak.hash()
    }
}
