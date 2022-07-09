#![forbid(unsafe_code)]

use anyhow::Result;
use scrabert::get_summary;

#[tokio::main]
async fn main() -> Result<()> {
    let tmp = get_summary().await?;
    println!("{}", tmp);
    // let summaries = get_summaries(contents).await?;
    // for s in summaries {
    //     println!("{}", s);
    // }let mut tmp = Vec::new();
    // tmp.push("Where is Amy?".to_owned());
    // tmp.push("Amy is in Vancouver.".to_owned());
    // let answer = get_answers(tmp).await?;
    // println!("{:?}", answer);
    // tmp = Vec::new();
    // tmp.push("I like cats!".to_owned());
    // let response = get_response(tmp).await?;
    // println!("{:?}", response);
    Ok(())
}