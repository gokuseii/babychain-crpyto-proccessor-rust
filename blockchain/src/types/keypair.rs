use ed25519_dalek::{ExpandedSecretKey, Keypair, PublicKey, SecretKey, Verifier};
use hex;
use keccak_rust::Keccak;

use crate::traits::Hashable;
use crate::types::signature::Signature;
use crate::types::Hash;

#[derive(Debug)]
pub struct KeyPair {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Clone for KeyPair {
    fn clone(&self) -> Self {
        let private_key = SecretKey::from_bytes(self.private_key.as_bytes()).unwrap();
        let public_key = PublicKey::from_bytes(self.public_key.as_bytes()).unwrap();
        Self {
            private_key,
            public_key,
        }
    }
}

impl KeyPair {
    pub fn new() -> Self {
        Self::generate()
    }

    pub fn from(private_key: SecretKey, public_key: PublicKey) -> Self {
        KeyPair {
            private_key,
            public_key,
        }
    }

    pub fn generate() -> Self {
        let csprng = &mut rand::rngs::OsRng {};
        let keypair = Keypair::generate(csprng);

        KeyPair {
            private_key: keypair.secret,
            public_key: keypair.public,
        }
    }

    pub fn sign(&self, message: &String) -> Signature {
        let exp = ExpandedSecretKey::from(&self.private_key);
        Signature(exp.sign(message.as_bytes(), &self.public_key))
    }

    pub fn verify(&self, message: &String, sig: &Signature) -> bool {
        self.public_key.verify(message.as_bytes(), &sig.0).is_ok()
    }

    pub fn print_keys(&self) {
        println!(
            "Private Key: 0x{}",
            hex::encode(self.private_key.as_bytes())
        );
        println!("Public Key: 0x{}", hex::encode(self.public_key.as_bytes()));
    }
}

impl Hashable for KeyPair {
    fn hash(&self) -> Hash {
        let mut keccak = Keccak::new(256);
        keccak.update(&hex::encode(self.public_key.as_bytes()));
        keccak.hash()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::keypair::KeyPair;

    #[test]
    fn print_keys() {
        let keypair = KeyPair::new();
        keypair.print_keys()
    }

    #[test]
    fn sign() {
        let keypair = KeyPair::new();
        let message = "Hello World!".to_string();
        let sig = keypair.sign(&message);
        sig.print();
        assert!(keypair.verify(&message, &sig));
    }

    #[test]
    fn sign_fail() {
        let keypair1 = KeyPair::new();
        let keypair2 = KeyPair::new();

        let message = "Hello World!".to_string();
        let sig = keypair1.sign(&message);

        assert_eq!(false, keypair2.verify(&message, &sig));
    }
}
