//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use alloy::providers::Provider;
use alloy::signers::{local::PrivateKeySigner, Signer};
use alloy::{
    primitives::address,
    providers::{ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
    sol,
};
use alloy_primitives::{
    utils::{format_ether, parse_ether},
    U256,
};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Fallback,
    "../client/src/contracts/out/Fallback.sol/Fallback.json"
);

pub async fn solution_fallback() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "http://localhost:8545";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
    let accounts = provider.get_accounts().await?;

    let contract = Fallback::deploy(&provider).await?;

    println!("contract.address(): {}", contract.address());

    let contract_owner = contract.owner().call().await?;

    println!("contract_owner is {contract_owner}");

    let sent_val = parse_ether("0.00009").unwrap();

    let set_value_call = contract.contribute();
    let calldata = set_value_call.calldata().to_owned();
    let tx = TransactionRequest::default()
        .from(accounts[1])
        .to(contract.address().clone())
        .value(sent_val)
        .input(calldata.into());
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Transaction hash: {}", pending_tx.tx_hash());

    let contribution = contract.getContribution().call().await?;
    println!("current contribution: {}", contribution);

    let owner = contract.owner().call().await?;
    println!("current owner: {}", owner);

    let tx = TransactionRequest::default()
        .from(accounts[1])
        .to(contract.address().clone())
        .value(sent_val);
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Transaction hash: {}", pending_tx.tx_hash());

    let owner = contract.owner().call().await?;
    println!("current owner: {}", owner);
    Ok(())
}
