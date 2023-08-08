// src/core/mod.rs
pub mod block;
pub mod chain;
pub mod error;

// Re-export for cleaner imports
pub use block::{Block, BlockHeader};
pub use chain::Blockchain;
pub use error::BlockchainError;
