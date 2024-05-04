use std::env;
use serenity::model::channel::Message;
use serenity::prelude::{Client, Context, EventHandler};
use serenity::async_trait;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message){
		msg.channel_id.say(&ctx, msg.content);
	}
 }
 
#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("");
    let mut client = Client::builder(token).event_handler(Handler).await?;
    if let Err(why) = client.start().await {
        println!("問題→{why:?}");
    }
}
