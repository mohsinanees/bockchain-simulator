// src/core/block.rs
use crate::utils::crypto::{hash, Hashable};
use serde::{Deserialize, Serialize};

/// Block header (metadata)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlockHeader {
    pub version: u32,            // Protocol version
    pub prev_block_hash: String, // SHA-256 hash of previous block
    pub merkle_root: String,     // Merkle root of transactions
    pub timestamp: u64,          // Unix timestamp
    pub bits: u32,               // Difficulty target
    pub nonce: u64,              // Proof-of-work counter
}

/// Full block structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>, // We'll define Transaction later
    pub hash: String,                   // Block hash (header + transactions)
}

impl Block {
    /// Creates the genesis block (hardcoded first block)
    pub fn genesis() -> Self {
        let header = BlockHeader {
            version: 1,
            prev_block_hash: String::from("0"),
            merkle_root: String::from("0"),
            timestamp: 1234567890,
            bits: 0x1e0ffff0, // Example difficulty
            nonce: 0,
        };
        Block::new(header, vec![]) // Genesis has no transactions
    }

    /// Creates a new block (without mining)
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        let hash = Self::calculate_hash(&header, &transactions);
        Self {
            header,
            transactions,
            hash,
        }
    }

    /// Mines a new block with Proof-of-Work
    pub fn mine(header: BlockHeader, transactions: Vec<Transaction>, difficulty: u32) -> Self {
        let mut header = header;
        let pow = ProofOfWork::new(difficulty);
        let transactions_hash = Self::hash_transactions(&transactions);

        let hash = pow.mine_block(&mut header, &transactions_hash);

        Self {
            header,
            transactions,
            hash,
        }
    }

    /// Hashes all transactions for the block header
    fn hash_transactions(transactions: &[Transaction]) -> String {
        let tx_hashes: Vec<String> = transactions.iter().map(|tx| tx.id.clone()).collect();
        compute_merkle_root(tx_hashes)
    }

    /// Calculates the block's SHA-256 hash
    pub fn calculate_hash(header: &BlockHeader, transactions: &[Transaction]) -> String {
        let header_hash = hash(header);
        let tx_hashes: Vec<String> = transactions.iter().map(|tx| tx.id.clone()).collect();
        let merkle_root = compute_merkle_root(tx_hashes);
        hash(&(header_hash + &merkle_root))
    }

    /// Validates the block's structure
    pub fn is_valid(&self) -> bool {
        self.hash == Self::calculate_hash(&self.header, &self.transactions)
    }
}

// Helper function for Merkle tree
fn compute_merkle_root(hashes: Vec<String>) -> String {
    // Simplified implementation (real one would hash pairs recursively)
    hash(&hashes.concat())
}
