use std::env;

extern crate futures;
extern crate tokio;
extern crate websocket;

use websocket::{ClientBuilder, OwnedMessage};

const base_url = "wss://gateway.discord.com/api/?v=v6";
const ws_url = base_url + "&encoding=json";


let mut args:Vec<String> = std::env::args().collect();

const TOKEN = args[1];
async fn message(){

}

async fn close(){
	
}

async fn open(s){
	await s.send("{op: 2, d: {token: 'Bot "+TOKEN+"', properties: {$os: 'linux', $device: 'chrome'}}}")
}

async fn connect(){
	let client = ClientBuilder::new(ws_url)
		.unwrap()
		.async_connect_secure()
		.and_then(|(s, _)| open(s));
	let _ = runtime.block_on(client).unwrap();
	return 
}

async fn main() {
 
	let mut runtime = tokio::runtime::current_thread::Builder::new()
		.build()
		.unwrap();
 	await connect();

}
