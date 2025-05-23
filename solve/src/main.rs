use eyre::{Ok, Result};

mod solutions;

#[tokio::main]
async fn main() -> Result<()> {
    // solutions::fallback::solution_fallback().await?;
    // solutions::fallout::solution_fallout().await?;
    // solutions::coin_flip::solution_coin_flip().await?;
    // solutions::telephone::solution_telephone().await?;
    // solutions::delegation::solution_delegation().await?;
    solutions::vault::solution_vault().await?;

    Ok(())
}
