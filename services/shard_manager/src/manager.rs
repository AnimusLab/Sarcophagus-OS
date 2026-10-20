use blake3::Hasher;
use serde::{Serialize, Deserialize};
use rand::RngCore;
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShardContainer {
    pub shard_id: String,
    pub payload_hash: String,
    pub encrypted_payload: Vec<u8>,
    pub is_decoy: bool,
}

pub struct ShardManager {
    node_secret_seed: [u8; 32],
}

impl ShardManager {
    pub fn new(seed: [u8; 32]) -> Self {
        Self { node_secret_seed: seed }
    }

    /// Chunks local state logs, hashes them, and packages them into a ring-signed container
    pub fn create_shard(&self, raw_data: &[u8], is_decoy: bool) -> ShardContainer {
        let mut hasher = Hasher::new();
        hasher.update(&self.node_secret_seed);
        hasher.update(raw_data);
        let hash = hasher.finalize();
        
        let shard_id = hex::encode(&hash.as_bytes()[..16]);
        let payload_hash = hex::encode(hash.as_bytes());

        // If it's a decoy shard, inject high-entropy random noise to defeat content profiling
        let payload = if is_decoy {
            let mut decoy_data = vec![0u8; raw_data.len()];
            OsRng.fill_bytes(&mut decoy_data);
            decoy_data
        } else {
            raw_data.to_vec()
        };

        ShardContainer {
            shard_id,
            payload_hash,
            encrypted_payload: payload,
            is_decoy,
        }
    }

    /// Generates a batch of synthetic decoy shards to mix with true traffic for K-anonymity
    pub fn generate_decoy_batch(&self, count: usize, chunk_size: usize) -> Vec<ShardContainer> {
        let mut batch = Vec::with_capacity(count);
        let mut dummy_data = vec![0u8; chunk_size];

        for _ in 0..count {
            OsRng.fill_bytes(&mut dummy_data);
            batch.push(self.create_shard(&dummy_data, true));
        }

        println!("[+] Generated {} synthetic decoy shards for K-anonymity pool mixing.", count);
        batch
    }
}