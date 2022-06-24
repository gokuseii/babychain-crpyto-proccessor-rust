use ed25519_dalek::{ExpandedSecretKey, Keypair, PublicKey, SecretKey, Verifier};
use hex;

use crate::types::signature::Signature;

pub struct KeyPair {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
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

    pub fn generate() -> KeyPair {
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

    fn verify(&self, message: &String, sig: &Signature) -> bool {
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
