use std::env;

pub extern crate futures;
pub extern crate url;

use self::client::builder::ClientBuilder;
use self::result::WebSocketResult;
 
#[tokio::main]
async fn main() {
 let mut args:Vec<String> = std::env::args().collect();
 
 let mut runtime = tokio::runtime::current_thread::Builder::new()
		.build()
		.unwrap();
 
 let runner = ClientBuilder::new("wss://gateway.discord.com/api/?v=v6&encoding=json")
  .unwrap()
  .async_connect_secure()
  let _ = runtime.block_on(runner).unwrap();
}
