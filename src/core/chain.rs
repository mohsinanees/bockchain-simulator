// src/core/chain.rs
use crate::core::block::{Block, BlockHeader};
use crate::core::error::BlockchainError;
use std::collections::HashMap;

/// The blockchain structure
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxo_set: HashMap<String, TransactionOutput>, // Unspent Transaction Outputs
    pub difficulty: u32,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new() -> Self {
        let genesis_block = Block::genesis();
        let mut utxo_set = HashMap::new();
        // Add genesis block outputs to UTXO set
        Self {
            blocks: vec![genesis_block],
            utxo_set,
            difficulty: 4, // Initial difficulty (number of leading zeros)
        }
    }

    pub fn add_block(&mut self, mut block: Block) -> Result<(), BlockchainError> {
        // 1. Validate Proof-of-Work
        let pow = ProofOfWork::new(self.difficulty);
        if !pow.validate(&block.hash) {
            return Err(BlockchainError::InvalidPoW);
        }

        // 2. Validate block linkage (previous hash)
        let last_block = self.blocks.last().ok_or(BlockchainError::EmptyChain)?;
        if block.header.prev_block_hash != last_block.hash {
            return Err(BlockchainError::InvalidBlockLink);
        }

        // 3. Validate block structure (e.g., hash correctness)
        if !block.is_valid() {
            return Err(BlockchainError::InvalidBlock);
        }

        // 4. Update UTXO set (if transactions exist)
        self.update_utxo_set(&block)?;

        // 5. Add block to chain
        self.blocks.push(block);

        // 6. Adjust difficulty for next block
        self.adjust_difficulty(block.header.timestamp);

        Ok(())
    }

    /// Validates the entire chain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let prev = &self.blocks[i - 1];
            let current = &self.blocks[i];
            if current.header.prev_block_hash != prev.hash || !current.is_valid() {
                return false;
            }
        }
        true
    }

    /// Updates the UTXO set after adding a block
    fn update_utxo_set(&mut self) -> Result<(), BlockchainError> {
        // Logic to remove spent inputs and add new outputs
        Ok(())
    }
}
