use std::error::Error;

const INPUT: &str = "https://adventofcode.com/2024/day/1/input";

#[tokio::main]
async fn main() -> Result<(), reqwet::Error> {
    let body = reqwest::get(INPUT)
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
}

