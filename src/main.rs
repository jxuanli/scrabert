#![forbid(unsafe_code)]

use anyhow::Result;
use scrabert::talk;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", talk("why is it called rust?").await?);
    Ok(())
}
