use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    server::ServerBuilder,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    number: String,
    hash: String,
    parentHash: String,
    timestamp: String,
    transactions: Vec<String>,
}

#[rpc(server)]
pub trait EthRpc {
    #[method(name = "eth_getBalance")]
    async fn get_balance(&self, address: String, block: String) -> RpcResult<String>;

    #[method(name = "eth_getBlockByNumber")]
    async fn get_block_by_number(
        &self,
        block_number: String,
        full_tx: bool,
    ) -> RpcResult<Option<Block>>;

    #[method(name = "eth_blockNumber")]
    async fn block_number(&self) -> RpcResult<String>;
}

pub struct EthRpcServer;

#[async_trait]
impl EthRpc for EthRpcServer {
    async fn get_balance(&self, _address: String, _block: String) -> RpcResult<String> {
        // Return a dummy balance of 1 ETH
        Ok("0xde0b6b3a7640000".to_string()) // 1 ETH in wei
    }

    async fn get_block_by_number(
        &self,
        block_number: String,
        _full_tx: bool,
    ) -> RpcResult<Option<Block>> {
        // Return a dummy block
        Ok(Some(Block {
            number: block_number,
            hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            parentHash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
                .to_string(),
            timestamp: "0x5f5e100".to_string(), // Current timestamp
            transactions: vec![],
        }))
    }

    async fn block_number(&self) -> RpcResult<String> {
        // Return a dummy block number
        Ok("0x1234".to_string())
    }
}

pub async fn start_rpc_server(addr: SocketAddr) -> anyhow::Result<()> {
    let server = ServerBuilder::default().build(addr).await?;

    let rpc = EthRpcServer;
    let handle = server.start(rpc.into_rpc())?;

    handle.stopped().await;
    Ok(())
}
