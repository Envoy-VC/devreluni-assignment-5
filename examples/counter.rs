use std::str::FromStr;

use dotenv::dotenv;

use alloy::{
    network::EthereumWallet, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, sol,
};
use eyre::eyre;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "abi/Counter.json"
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let private_key =
        std::env::var("PRIVATE_KEY").map_err(|_| eyre!("No {} env var set", "PRIVATE_KEY"))?;
    let rpc_url = std::env::var("RPC_URL").map_err(|_| eyre!("No {} env var set", "RPC_URL"))?;
    let contract_address = "0x7e32b54800705876d3b5cfbc7d9c226a211f7c1a";

    let signer: PrivateKeySigner = PrivateKeySigner::from_str(&private_key).unwrap();
    let wallet = EthereumWallet::from(signer.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url.parse()?);

    let counter = Counter::new(contract_address.parse()?, provider.clone());

    let mut count: U256 = counter.number().call().await?._0;

    println!("Initial Count = {:?}", count);

    // Set Number
    counter
        .setNumber(U256::from(10))
        .send()
        .await?
        .watch()
        .await?;

    count = counter.number().call().await?._0;
    println!("After Set Number Count = {:?}", count);

    // Multiply Number
    counter
        .mulNumber(U256::from(10))
        .send()
        .await?
        .watch()
        .await?;

    count = counter.number().call().await?._0;
    println!("After Multiply Number Count = {:?}", count);

    // Add Number
    counter
        .addNumber(U256::from(10))
        .send()
        .await?
        .watch()
        .await?;

    count = counter.number().call().await?._0;
    println!("After Add Number Count = {:?}", count);

    // Increment Number
    counter.increment().send().await?.watch().await?;
    count = counter.number().call().await?._0;
    println!("After Increment Count = {:?}", count);

    Ok(())
}
