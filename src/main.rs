use dotenv;
use std::env;
use discord_bot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env file not found");

    let token: String = env::var("TOKEN").unwrap();
    let mut client = discord_bot::clientBuilder(&token).await;

    if let Err(why) = client.start().await {
        print!("Client Error {:?}", why);
    }
}