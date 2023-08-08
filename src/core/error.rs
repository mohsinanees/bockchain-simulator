// src/core/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Empty blockchain")]
    EmptyChain,
    #[error("Invalid block linkage")]
    InvalidBlockLink,
    #[error("Invalid proof-of-work")]
    InvalidPoW,
    #[error("Invalid transaction")]
    InvalidTransaction,
}
