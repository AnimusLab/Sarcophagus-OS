use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Signature};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PulseIdentity {
    pub handle: String,
    pub public_key_hex: String,
}

pub struct IdentityManager {
    signing_key: SigningKey,
    pub handle: String,
}

impl IdentityManager {
    pub fn new(handle: &str) -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self {
            signing_key,
            handle: handle.to_string(),
        }
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn export_profile(&self) -> PulseIdentity {
        let pk_bytes = self.public_key().to_bytes();
        PulseIdentity {
            handle: self.handle.clone(),
            public_key_hex: hex::encode(pk_bytes),
        }
    }

    pub fn sign_packet(&self, payload: &[u8]) -> Signature {
        self.signing_key.sign(payload)
    }
}