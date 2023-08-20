// src/core/pow.rs
use crate::core::block::BlockHeader;
use crate::utils::crypto::hash;
use num_bigint::BigUint;
use num_traits::One;

#[derive(Debug)]
pub struct ProofOfWork {
    pub difficulty: u32, // Number of leading zeros required
}

impl ProofOfWork {
    /// Creates a new PoW instance with given difficulty
    pub fn new(difficulty: u32) -> Self {
        Self { difficulty }
    }

    /// Mines a block by finding a valid nonce
    pub fn mine_block(&self, header: &mut BlockHeader, transactions_hash: &str) -> String {
        let target = self.calculate_target();

        header.nonce = 0;
        loop {
            let hash = self.calculate_hash(header, transactions_hash);
            let hash_int = BigUint::from_bytes_be(&hex::decode(&hash).unwrap());

            if hash_int < target {
                return hash; // Valid hash found
            }

            header.nonce += 1;
        }
    }

    /// Validates if a block's hash meets the difficulty target
    pub fn validate(&self, block_hash: &str) -> bool {
        let target = self.calculate_target();
        let hash_int = BigUint::from_bytes_be(&hex::decode(block_hash).unwrap());
        hash_int < target
    }

    /// Calculates the target value for mining
    fn calculate_target(&self) -> BigUint {
        BigUint::one() << (256 - self.difficulty)
    }

    /// Computes the hash of a block header
    fn calculate_hash(&self, header: &BlockHeader, transactions_hash: &str) -> String {
        let data = format!(
            "{}{}{}{}{}",
            header.version,
            header.prev_block_hash,
            transactions_hash,
            header.timestamp,
            header.bits
        );
        hash(&data)
    }
}
