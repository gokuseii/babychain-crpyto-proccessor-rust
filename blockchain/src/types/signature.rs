#[derive(Debug, Clone)]
pub struct Signature(pub ed25519_dalek::Signature);

impl Signature {
    pub fn to_string(&self) -> String {
        format!("{}", hex::encode(self.0.as_ref()))
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}
