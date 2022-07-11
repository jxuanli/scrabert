#![forbid(unsafe_code)]

use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};
use scrabert::talk;

#[tokio::main]
async fn main() {
    loop {
        print!(">>>> ");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("failed to read line");

        let now = Instant::now();
        let res = talk(user_text.as_str()).await.unwrap();
        println!("\n\n{}\nThis thinking took {}s to complete. \n\n", res, now.elapsed().as_secs());
    }
}
