use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use rand::rngs::OsRng;
use anyhow::Result;

pub const FIXED_PACKET_SIZE: usize = 2048; // Uniform block sizing to defeat size fingerprinting

pub struct PagerMessagePacket {
    pub ciphertext: Vec<u8>,
}

impl PagerMessagePacket {
    /// Pads and encrypts a raw message payload to a fixed cryptographic block size
    pub fn encrypt_and_pad(shared_secret: &[u8], plaintext: &[u8]) -> Result<Self> {
        if shared_secret.len() < 32 {
            anyhow::bail!("Invalid shared secret length");
        }

        let key = Key::<Aes256Gcm>::from_slice(&shared_secret[..32]);
        let cipher = Aes256Gcm::new(key);

        let mut rng = OsRng;
        let mut nonce_bytes = [0u8; 12];
        rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt payload
        let mut ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("AES encryption failed: {:?}", e))?;

        let payload_len = ciphertext.len() as u16;

        // Packet Layout: [12 bytes nonce] + [2 bytes length header] + [ciphertext...] + [padding zeros]
        let mut packet = Vec::with_capacity(FIXED_PACKET_SIZE);
        packet.extend_from_slice(&nonce_bytes);
        packet.extend_from_slice(&payload_len.to_le_bytes());
        packet.append(&mut ciphertext);

        if packet.len() > FIXED_PACKET_SIZE {
            anyhow::bail!("Payload exceeds maximum fixed container block size");
        }

        // Pad remainder with cryptographic zeros to ensure uniform 2048-byte blocks
        packet.resize(FIXED_PACKET_SIZE, 0u8);

        Ok(Self {
            ciphertext: packet,
        })
    }

    /// Decrypts and unpads an incoming 2048-byte cryptographic packet block
    pub fn decrypt_and_unpad(shared_secret: &[u8], packet_block: &[u8]) -> Result<Vec<u8>> {
        if shared_secret.len() < 32 || packet_block.len() != FIXED_PACKET_SIZE {
            anyhow::bail!("Invalid shared secret or packet size mismatch");
        }

        let key = Key::<Aes256Gcm>::from_slice(&shared_secret[..32]);
        let cipher = Aes256Gcm::new(key);

        let nonce_bytes = &packet_block[..12];
        let len_bytes = &packet_block[12..14];
        let payload_len = u16::from_le_bytes([len_bytes[0], len_bytes[1]]) as usize;

        if 14 + payload_len > FIXED_PACKET_SIZE {
            anyhow::bail!("Malformed packet length header");
        }

        let ciphertext = &packet_block[14..14 + payload_len];
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("AES decryption failed: {:?}", e))?;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_padding_and_encryption() {
        let secret = [0x42u8; 32];
        let plaintext = b"Test secure ping message";
        
        let packet = PagerMessagePacket::encrypt_and_pad(&secret, plaintext).unwrap();
        assert_eq!(packet.ciphertext.len(), FIXED_PACKET_SIZE);

        let decrypted = PagerMessagePacket::decrypt_and_unpad(&secret, &packet.ciphertext).unwrap();
        assert_eq!(&decrypted, plaintext);
    }
}