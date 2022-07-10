#![forbid(unsafe_code)]

use anyhow::Result;
use scrabert::{get_answer, get_response};

#[tokio::main]
async fn main() -> Result<()> {
    test().await?;
    Ok(())
}

async fn test() -> Result<()> {
    let answer = get_answer("why is it called rust?").await?;
    println!("{:?}", answer);

    let mut tmp = Vec::new();
    tmp.push("I like cats!".to_owned());
    let mut k = Vec::new();
    k.push(tmp);
    let response = get_response(k).await?;
    println!("{:?}", response);
    Ok(())
}
