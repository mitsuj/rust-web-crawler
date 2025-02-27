mod fetcher;
mod frontier;
mod parser;
mod politeness;
mod storage;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting web crawler...");

    Ok(())
}
