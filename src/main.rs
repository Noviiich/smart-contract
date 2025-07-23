use ethers::{
    prelude::*,
};
use smart_contract::config::Config;
use smart_contract::blockchain::BlockChainClient;
use smart_contract::token::TokenClient;

const RPC_URL: &str = "https://ethereum-sepolia-rpc.publicnode.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::from_env().expect("Failed to load configuration from environment variables");
    let from = cfg.sender.address;
    let to = cfg.recipient.address;

    let blockchain_client = BlockChainClient::new(RPC_URL).await?;

    let token_client_sender = TokenClient::new(
        blockchain_client.provider.clone(),
        cfg.sender.private_key,
        cfg.token_address,
        blockchain_client.chan_id,
    )?;
    let token_client_recipient = TokenClient::new(
        blockchain_client.provider.clone(),
        cfg.recipient.private_key,
        cfg.token_address,
        blockchain_client.chan_id,
    )?;

    // Баланс отправителя
    let balance = token_client_sender.get_balance(from).await?;
    println!("Balance of {}: {:?}", from, balance);

    // Баланс Получателя
    let balance = token_client_recipient.get_balance(to).await?;
    println!("Balance of {}: {:?}", to, balance);

    let whole_amount: u64 = 1;
    let decimals: u8 = 18;
    let decimal_amount = U256::from(whole_amount) * U256::exp10(decimals as usize);

    // Отправка токенов от sender к recipient
    token_client_sender.transfer(to, decimal_amount).await?;
    // Разрешить отправлять токены от sender к recipient
    token_client_sender.approve(to, decimal_amount).await?;
    // Проверка разрешения от sender к recipient
    let allowance = token_client_recipient.allowance(from, to).await?;
    // Отправка токенов от sender к recipient через transfer_from
    token_client_recipient.transfer_from(from, to, decimal_amount).await?;

    Ok(())
}


