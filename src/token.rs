use ethers::{
    core::{types::Address}, 
    middleware::SignerMiddleware, 
    prelude::*, providers::{Http, Provider}, 
    signers::{LocalWallet},
};
use std::{sync::Arc, time::Duration};

abigen!(
    ERC20Contract,
    r#"[
        function balanceOf(address account) public view returns (uint256)
        function transfer(address recipient, uint256 amount) public returns (bool)
        function allowance(address owner, address spender) public view returns (uint256)
        function approve(address spender, uint256 amount) public returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) public returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

pub struct TokenClient {
    pub contract: ERC20Contract<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    pub wallet: LocalWallet,
}
impl TokenClient
{
    pub fn new(provider: Arc<Provider<Http>>, private_key: String, token_address: Address, chain_id: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let signer = Arc::new(SignerMiddleware::new(provider, wallet.clone()));
        let contract = ERC20Contract::new(token_address, signer);
        Ok(Self { contract, wallet })
    }

    pub async fn get_balance(&self, account_address: Address) -> Result<U256, Box<dyn std::error::Error>> {
        let balance = self.contract.balance_of(account_address).call().await?;
        Ok(balance)
    }

    pub async fn transfer(&self, to: Address, amount: U256) -> Result<(), Box<dyn std::error::Error>> {
        match self.contract.transfer(to, amount).send().await {
            Ok(pending_tx) => {
                println!("Транзакция отправлена, ожидаем подтверждения...");

                match pending_tx.await {
                    Ok(_) => {
                        println!("Отправка токенов с transfer завершилась");
                        Ok(())
                    }
                    Err(err) => {
                        println!("Ошибка подтверждения транзакции для {:?}", err);
                        Err(err.into())
                }
                }
            }
            Err(err) => {
                println!("Error during transfer: {:?}", err);
                Err(err.into())
            }
        }
    }
    pub async fn approve(&self, spender: Address, amount: U256) -> Result<(), Box<dyn std::error::Error>> {
       match self.contract.approve(spender, amount).send().await {
            Ok(pending_tx) => {
                println!("Транзакция approve отправлена, ожидаем подтверждения...");
                
                match pending_tx.await {
                    Ok(receipt) => {
                        println!("Разрешение успешно дано для {}", spender);
                        if let Some(receipt) = &receipt {
                            println!("Блок: {:?}, Gas использовано: {:?}", 
                                receipt.block_number, 
                                receipt.gas_used
                            );
                        }
                        Ok(())
                    }
                    Err(err) => {
                        println!("Ошибка подтверждения транзакции approve для {}: {:?}", spender, err);
                        Err(err.into())
                    }
                }
            }
            Err(err) => {
                println!("Ошибка отправки транзакции approve для {}: {:?}", spender, err);
                Err(err.into())
            }
        }
    }
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Box<dyn std::error::Error>> {
        match self.contract.allowance(owner, spender).call().await {
            Ok(amount) => {
                println!("Allowance от {} на {}: {}", owner, spender, amount);
                Ok(amount)
            }
            Err(err) => {
                println!("Ошибка получения allowance от {} на {}: {:?}", owner, spender, err);
                Err(err.into())
            }
        }
    }
    pub async fn transfer_from(
        &self,
        from : Address,
        to: Address, 
        amount: U256
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.contract.transfer_from(from, to, amount).send().await {
            Ok(pending_tx) => {
                println!("Транзакция transferFrom отправлена, ожидаем подтверждения...");
                
                match pending_tx.await {
                    Ok(receipt) => {
                        println!("TransferFrom успешно выполнен от {} к {} на сумму {}", from, to, amount);
                        if let Some(receipt) = &receipt {
                            println!("Блок: {:?}, Gas использовано: {:?}", 
                                receipt.block_number, 
                                receipt.gas_used
                            );
                        }
                        Ok(())
                    }
                    Err(err) => {
                        println!("Ошибка подтверждения транзакции transferFrom: {:?}", err);
                        Err(err.into())
                    }
                }
            }
            Err(err) => {
                println!("Ошибка отправки транзакции transferFrom: {:?}", err);
                Err(err.into())
            }
        }
    }
}
