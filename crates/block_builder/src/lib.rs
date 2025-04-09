use alloy::primitives::{Address, B256, U256};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tx::tx::Tx;

#[derive(Debug, Clone)]
pub struct Block {
    pub number: U256,
    pub hash: B256,
    pub parent_hash: B256,
    pub nonce: u64,
    pub timestamp: u64,
    pub transactions: Vec<Tx>,
    pub state_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: Bytes,
    pub gas_used: U256,
    pub gas_limit: U256,
    pub base_fee_per_gas: Option<U256>,
    pub miner: Address,
}

impl Block {
    pub fn new(
        number: U256,
        parent_hash: B256,
        timestamp: u64,
        transactions: Vec<Tx>,
        miner: Address,
    ) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(number.to_be_bytes::<32>());
        hasher.update(parent_hash.as_slice());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(miner.as_slice());
        
        for tx in &transactions {
            hasher.update(tx.tx_hash());
        }

        let hash = B256::from_slice(&hasher.finalize());

        Self {
            number,
            hash,
            parent_hash,
            nonce: 0,
            timestamp,
            transactions,
            state_root: B256::ZERO,
            receipts_root: B256::ZERO,
            logs_bloom: Bytes::new(),
            gas_used: U256::ZERO,
            gas_limit: U256::from(30_000_000),
            base_fee_per_gas: Some(U256::from(1_000_000_000)),
            miner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockBuilder {
    blocks: Arc<RwLock<HashMap<U256, Block>>>,
    latest_block_number: Arc<RwLock<U256>>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            blocks: Arc::new(RwLock::new(HashMap::new())),
            latest_block_number: Arc::new(RwLock::new(U256::ZERO)),
        }
    }

    pub async fn create_block(
        &self,
        transactions: Vec<Tx>,
        miner: Address,
    ) -> anyhow::Result<Block> {
        let mut blocks = self.blocks.write().await;
        let mut latest_number = self.latest_block_number.write().await;

        let parent_hash = if *latest_number == U256::ZERO {
            B256::ZERO
        } else {
            blocks.get(&(*latest_number - U256::from(1)))
                .map(|block| block.hash)
                .unwrap_or(B256::ZERO)
        };

        let block = Block::new(
            *latest_number,
            parent_hash,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            miner,
        );

        blocks.insert(*latest_number, block.clone());
        *latest_number += U256::from(1);

        Ok(block)
    }

    pub async fn get_block(&self, number: U256) -> Option<Block> {
        let blocks = self.blocks.read().await;
        blocks.get(&number).cloned()
    }

    pub async fn get_latest_block(&self) -> Option<Block> {
        let latest_number = *self.latest_block_number.read().await;
        if latest_number == U256::ZERO {
            None
        } else {
            self.get_block(latest_number - U256::from(1)).await
        }
    }

    pub async fn get_latest_block_number(&self) -> U256 {
        *self.latest_block_number.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;

    #[tokio::test]
    async fn test_block_creation() {
        let block_builder = BlockBuilder::new();
        let miner = PrivateKeySigner::random().address();

        // Create first block
        let block1 = block_builder
            .create_block(Vec::new(), miner)
            .await
            .unwrap();

        assert_eq!(block1.number, U256::ZERO);
        assert_eq!(block1.parent_hash, B256::ZERO);

        // Create second block
        let block2 = block_builder
            .create_block(Vec::new(), miner)
            .await
            .unwrap();

        assert_eq!(block2.number, U256::from(1));
        assert_eq!(block2.parent_hash, block1.hash);

        // Verify latest block
        let latest_block = block_builder.get_latest_block().await.unwrap();
        assert_eq!(latest_block.hash, block2.hash);
        assert_eq!(block_builder.get_latest_block_number().await, U256::from(2));
    }

    #[tokio::test]
    async fn test_block_retrieval() {
        let block_builder = BlockBuilder::new();
        let miner = PrivateKeySigner::random().address();

        let block = block_builder
            .create_block(Vec::new(), miner)
            .await
            .unwrap();

        let retrieved_block = block_builder.get_block(U256::ZERO).await.unwrap();
        assert_eq!(retrieved_block.hash, block.hash);
    }
}
