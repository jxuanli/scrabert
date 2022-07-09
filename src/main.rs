#![forbid(unsafe_code)]

use anyhow::Result;
use scrabert::{get_answer, get_response, get_summary};

#[tokio::main]
async fn main() -> Result<()> {
    // let tmp = get_summary().await?;
    // println!("{}", tmp);
    test().await?;
    Ok(())
}

async fn test() {
    let mut tmp = Vec::new();
    tmp.push("Where is Amy?".to_owned());
    tmp.push("Amy is in Vancouver.".to_owned());
    let mut k = Vec::new();
    k.push(tmp);
    let answer = get_answer(k).await?;
    println!("{:?}", answer);

    tmp = Vec::new();
    tmp.push("I like cats!".to_owned());
    k = Vec::new();
    k.push(tmp);
    let response = get_response(k).await?;
    println!("{:?}", response);
}
