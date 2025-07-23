use std::{
    sync::Arc,
    time::Duration,
};
use ethers::{
    prelude::*, providers::{Http, Provider}, 
};

pub struct BlockChainClient {
    pub provider: Arc<Provider<Http>>,
    pub rpc_url: String,
    pub chan_id: u64,
}

impl BlockChainClient {
    pub async fn new(rpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from(rpc_url)?.interval(Duration::from_millis(10u64));
        let chain_id = provider.get_chainid().await?;
        Ok(Self {
            provider: Arc::new(provider),
            rpc_url: rpc_url.to_string(),
            chan_id: chain_id.as_u64(),
        })
    }
}