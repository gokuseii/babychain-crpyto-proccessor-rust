use std::collections::HashMap;
use std::ops::Sub;

use crate::traits::Hashable;
use crate::types::block::Block;
use crate::types::transaction::Transaction;
use crate::types::{AccountId, Balance, Error};

pub struct Blockchain {
    pub(crate) coin_database: HashMap<AccountId, Balance>,
    pub(crate) history: Vec<Block>,
    pub(crate) tx_database: Vec<Transaction>,
    faucet_coins: Balance,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis = Block::new(None, Vec::default());
        Blockchain {
            coin_database: HashMap::default(),
            history: vec![genesis],
            tx_database: Vec::default(),
            faucet_coins: 100_000_000,
        }
    }

    fn get_token_from_faucet(&mut self, to: AccountId, amount: Balance) {
        *self.coin_database.entry(to).or_insert(0) += amount;
        self.faucet_coins = self.faucet_coins.sub(amount);
    }

    fn validate(&mut self, block: Block) -> Result<(), Error> {
        let is_genesis = self.history.is_empty();

        if !is_genesis {
            let hash = block
                .clone()
                .prev
                .expect("Block does not have a hash for prev block");
            let prev_block = self.history.last().unwrap().clone();
            if prev_block.hash() != hash {
                return Err("Incorrect hash for prev block".to_string());
            }
        }

        let backup = self.coin_database.clone();
        for block_transaction in block.transactions.clone() {
            if let Err(error) = block_transaction.validate(self) {
                self.coin_database = backup.clone();
                return Err(format!(
                    "Error during transaction validation, message error: {}",
                    error
                ));
            }
        }
        self.history.push(block);
        Ok(())
    }

    pub fn balance(&self, account_id: AccountId) -> Balance {
        let balance = self.coin_database.get(account_id.as_str());
        match balance {
            Some(balance) => balance.clone(),
            _ => 0,
        }
    }

    pub(crate) fn add_to_balance(
        &mut self,
        account_id: AccountId,
        amount: Balance,
    ) -> Result<(), Error> {
        let new_balance = self.coin_database.entry(account_id).or_insert(0);
        match new_balance.checked_add(amount) {
            Some(_) => {
                *new_balance += amount;
            }
            _ => return Err("Overflow balance".to_string()),
        }
        Ok(())
    }

    pub(crate) fn sub_from_balance(
        &mut self,
        account_id: AccountId,
        amount: Balance,
    ) -> Result<(), Error> {
        let new_balance = self.coin_database.entry(account_id).or_insert(0);
        match new_balance.checked_sub(amount) {
            Some(_) => {
                *new_balance -= amount;
            }
            _ => return Err("Overflow balance".to_string()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Hashable;
    use crate::types::block::Block;
    use crate::types::blockhain::Blockchain;
    use crate::types::operation::Operation;
    use crate::types::transaction::Transaction;
    use crate::types::{Account, Hash};

    #[test]
    fn create_blockchain() {
        let chain = Blockchain::new();
        assert_eq!(1, chain.history.len());
    }

    #[test]
    fn validate_block_with_transaction() {
        let mut chain = Blockchain::new();

        let alice = Account::new();
        let bob = Account::new();

        chain.get_token_from_faucet(alice.clone().id, 100_000);

        let operation = Operation::new(alice.clone(), bob.clone(), 10_000, None);
        let transaction = Transaction::new(1, vec![operation]);
        let prev_hash = chain.history.last().unwrap().hash();
        let block = Block::new(Some(prev_hash), vec![transaction]);

        assert_eq!(100_000, chain.balance(alice.clone().id));
        assert_eq!(0, chain.balance(bob.clone().id));

        assert!(chain.validate(block).is_ok());
        assert_eq!(90_000, chain.balance(alice.clone().id));
        assert_eq!(10_000, chain.balance(bob.clone().id));
        assert_eq!(2, chain.history.len());
    }

    #[test]
    fn failed_incorrect_hash_prev_block() {
        let mut chain = Blockchain::new();

        let alice = Account::new();
        let bob = Account::new();

        chain.get_token_from_faucet(alice.clone().id, 100_000);

        let operation = Operation::new(alice.clone(), bob.clone(), 10_000, None);
        let transaction = Transaction::new(1, vec![operation]);
        let fake_hash: Hash = "Fake Hash".to_string();
        let block = Block::new(Some(fake_hash), vec![transaction]);

        assert_eq!(100_000, chain.balance(alice.clone().id));
        assert_eq!(0, chain.balance(bob.clone().id));

        assert_eq!(
            "Incorrect hash for prev block",
            chain.validate(block).err().unwrap()
        );

        assert_eq!(100_000, chain.balance(alice.clone().id));
        assert_eq!(0, chain.balance(bob.clone().id));
        assert_eq!(1, chain.history.len());
    }

    #[test]
    fn failed_insufficient_balance() {
        let mut chain = Blockchain::new();

        let alice = Account::new();
        let bob = Account::new();

        chain.get_token_from_faucet(alice.clone().id, 100_000);

        let operation = Operation::new(alice.clone(), bob.clone(), 1_000_000, None);
        let transaction = Transaction::new(1, vec![operation]);

        let prev_hash = chain.history.last().unwrap().hash();
        let block = Block::new(Some(prev_hash), vec![transaction]);

        assert_eq!(100_000, chain.balance(alice.clone().id));
        assert_eq!(0, chain.balance(bob.clone().id));

        assert_eq!(
            "Error during transaction validation, message error: Insufficient balance",
            chain.validate(block).err().unwrap()
        );

        assert_eq!(100_000, chain.balance(alice.clone().id));
        assert_eq!(0, chain.balance(bob.clone().id));
        assert_eq!(1, chain.history.len());
    }
}
