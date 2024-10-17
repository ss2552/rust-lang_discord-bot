use std::env;
use serenity::prelude::{Client};
 
#[tokio::main]
async fn main() {
    let mut args = std::env::args.collect();
    let mut client = Client::builder(args[1]).event_handler(Handler).await?;
    client.start().await?;
}
