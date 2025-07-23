use ethers::core::{types::Address};
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub sender: Wallet,
    pub recipient: Wallet,
    pub token_address: Address,
}

pub struct Wallet {
    pub address: Address,
    pub private_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let sender_private_key = env::var("SENDER_PRIVATE_KEY")?;
        let sender_address = env::var("SENDER_ADDRESS")?;

        let recipient_private_key = env::var("RECIPIENT_PRIVATE_KEY")?;
        let recipient_address = env::var("RECIPIENT_ADDRESS")?;

        let token_address = env::var("TOKEN_ADDRESS")?.parse::<Address>()?;

        let sender_wallet = Wallet {
            address: sender_address.parse::<Address>()?,
            private_key: sender_private_key,
        };
        let recipient_wallet = Wallet {
            address: recipient_address.parse::<Address>()?,
            private_key: recipient_private_key,
        };

        Ok(Self {
            sender: sender_wallet,
            recipient: recipient_wallet,
            token_address,
        })
    }
}