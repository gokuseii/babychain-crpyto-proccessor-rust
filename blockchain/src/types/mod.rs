pub use account::Account;
pub use keypair::KeyPair;
pub use signature::Signature;

mod account;
mod block;
mod blockhain;
mod keypair;
mod operation;
mod signature;
mod transaction;

pub type AccountId = String;
pub type Balance = u128;
pub type Hash = String;
pub type Error = String;
