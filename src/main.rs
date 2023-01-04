use discord_bot;
use dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env file not found");

    let token: String = env::var("TOKEN").unwrap();
    let mut client = discord_bot::client_builder(&token).await;

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("Received Ctrl-C, shutting down.");
}
