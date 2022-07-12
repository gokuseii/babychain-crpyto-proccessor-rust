use crate::traits::Hashable;
use crate::types::operation::Operation;
use crate::types::{AccountId, Balance, KeyPair, Signature};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: AccountId,
    wallet: Vec<KeyPair>,
    balance: Balance,
}

impl Account {
    pub fn new() -> Self {
        Self::generate()
    }

    fn generate() -> Self {
        let keypair = KeyPair::generate();
        Account {
            id: keypair.hash(),
            wallet: vec![keypair],
            balance: 0,
        }
    }

    fn add_keypair(&mut self, keypair: KeyPair) {
        self.wallet.push(keypair);
    }

    fn update_balance(&mut self, balance: Balance) {
        self.balance = balance;
    }

    pub fn balance(&self) -> Balance {
        self.balance
    }

    pub fn print_balance(&self) {
        println!("{}", self.balance);
    }

    fn wallet(&self, index: usize) -> Option<&KeyPair> {
        self.wallet.get(index)
    }

    pub fn sign(&self, message: &String, index: Option<usize>) -> Signature {
        let wallet = self
            .wallet(index.unwrap_or(0))
            .expect("Incorrect index of wallet");
        wallet.sign(message)
    }

    pub fn verify(&self, message: &String, signature: &Signature, index: Option<usize>) -> bool {
        let wallet = self
            .wallet(index.unwrap_or(0))
            .expect("Incorrect index of wallet");
        wallet.verify(message, signature)
    }

    fn create_operation(
        &self,
        receiver: Account,
        amount: Balance,
        index_wallet: Option<usize>,
    ) -> Operation {
        Operation::new(self.clone(), receiver, amount, index_wallet)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Hashable;

    use super::Account;

    #[test]
    fn create_account() {
        let alice = Account::new();
        let keypair = alice.wallet(0).unwrap();

        let account_id = keypair.hash();

        assert_eq!(0, alice.balance());
        assert_eq!(account_id, alice.id);
    }

    #[test]
    fn operation() {
        let mut alice = Account::new();
        let bob = Account::new();
        let amount = 1000;

        alice.update_balance(10000);

        let operation = alice.create_operation(bob.clone(), amount, None);
        let signature = alice.sign(&format!("{:?}", (alice.clone().id, bob.id, amount)), None);

        assert_eq!(10000, alice.balance());
        assert_eq!(amount, operation.amount());
        assert_eq!(signature.to_string(), operation.signature().to_string());
        assert!(operation.verify(None));
    }

    #[test]
    fn operation_incorrect() {
        let mut alice = Account::new();
        let bob = Account::new();
        let amount = 100000;

        alice.update_balance(10000);

        let operation = alice.create_operation(bob.clone(), amount, None);
        let signature = alice.sign(&format!("fake-data"), None);

        assert_ne!(signature.to_string(), operation.signature().to_string());
        assert!(!operation.verify(None));
    }
}
