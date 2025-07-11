use std::env;
// ウェブソケット
use tokio_tungstenite::{connect_async, tungstenite::{Message}};
use futures_util::{StreamExt, SinkExt};
// JSON
//use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

async fn start(){

    let discord_gateway_url = "ws://gateway.discord.gg/?v=10&encoding=json";
    
    println!("接続開始... {}", discord_gateway_url);
    
    let (ws_stream, _) = connect_async(discord_gateway_url)
        .await
        .expect("[E] 接続失敗!");
        
    println!("接続完了");

    let (mut write, mut read) = ws_stream.split();
    
    // on_open
    let open_body = r#"{
        "op": 2,
        "d": {
            "token":  "",
            "intents": 4,
            "properties": {
                "os": "linux",
                "device: "pc"
            }
        }
    }"#;

    write.send(Message::text(open_body))
        .await
        .expect("[E] openの送信失敗");
        
    let _ = tokio::try_join!(tokio::spawn(async move {
        while let Some(message) = read.next().await{
            match message{
                Ok(body) => {
                // let mut interaction : Value = serde_json::from_str(body).unwrap();
                 println!("受信");   
                },
                Err(e) => {
                    println!("[E] 受信の失敗 {}", e)
                }
            }
        }
    }));
}

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(start());
}
