mod cli;
mod rpc;

// 1. Setup node: ./target/release/node-template --dev --tmp --rpc-methods=Unsafe --rpc-cors all
// --rpc-external --ws-external 2: ws://192.168.31.52:9944

use cli::app::App;
use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::client::{Client, ClientBuilder, ClientT},
	rpc_params,
};
use rustyline::{error::ReadlineError, DefaultEditor};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// TODO: Query the block number of the Polkadot
	// let addr = "127.0.0.1:9944";
	// let uri: Uri = format!("ws://{}", addr).parse()?;
	// let (tx, rx) = WsTransportClientBuilder::default().build(uri).await?;
	// let client = ClientBuilder::default().build_with_tokio(tx, rx);
	// let response: String = client.request("system_version", rpc_params![]).await?;
	// println!("bear: the result is {:?}", response);
	let app = App::parse();

	let mut rl = DefaultEditor::new()?;
	loop {
		let readline = rl.readline(">> ");
		match readline {
			Ok(line) => {
				rl.add_history_entry(line.as_str());
				// handle command
				println!("Line: {}", line);
			},
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			},
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			},
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			},
		}
	}

	Ok(())
}
