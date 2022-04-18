use reqwest;
use std::error::Error;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url).await?;

    let content = response.text().await?;
    print!("{}", content);

    Ok(())
}
