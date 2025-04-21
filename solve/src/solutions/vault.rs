use alloy::providers::{Provider, ProviderBuilder};
use alloy::{rpc::types::TransactionRequest, sol};
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_primitives::FixedBytes;
use alloy_primitives::{
    utils::parse_ether,
    {address, keccak256, Address, U256},
};
use eyre::Result;
use rand::Rng;
use tokio::time::{interval, Duration};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Vault,
    "../client/src/contracts/out/Vault.sol/Vault.json"
);

pub async fn solution_vault() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
    let accounts = provider.get_accounts().await?;
    let sender = accounts[1];
    let mut rng = rand::rng();
    let random_bytes: [u8; 32] = rng.random();
    println!("Random [u8; 32]: 0x{}", hex::encode(random_bytes));

    let contract = Vault::deploy(&provider, FixedBytes::new(random_bytes)).await?;
    let got_password = provider
        .get_storage_at(contract.address().clone(), U256::from(1))
        .await?;
    println!("Storage slot 0: 0x{}", got_password.to_string());

    let locked = contract.locked().call().await?;
    println!("init locked: {}", locked);

    let call_builder = contract.unlock(got_password.into());
    let calldata = call_builder.calldata().to_owned();
    let tx = TransactionRequest::default()
        .from(sender)
        .to(contract.address().clone())
        .input(calldata.into());
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Transaction hash: {}", pending_tx.tx_hash());

    let locked = contract.locked().call().await?;
    println!("end locked: {}", locked);
    Ok(())
}
