const INPUT: &str = "https://adventofcode.com/2024/day/1/input";

async fn main() -> Result<> {
    let body = reqwest::get(INPUT)
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
}

