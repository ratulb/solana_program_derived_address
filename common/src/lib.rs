use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructionData {
    pub seed: String,
    pub bump_seed: u8,
    pub lamports: u64,
}

impl InstructionData {
    pub fn new(seed: &str, bump_seed: u8, lamports: u64) -> Self {
        Self {
            seed: String::from(seed),
            bump_seed,
            lamports,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let serialized = serde_json::to_string(self).expect("Failed serialization");
        serialized.as_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let deserialized = String::from_utf8(bytes.to_vec()).expect("Failed to convert!");
        let deserialized: Self =
            serde_json::from_str(&deserialized).expect("Failed deserialization");
        deserialized
    }
}
