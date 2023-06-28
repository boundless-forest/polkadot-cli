mod cli;
mod rpc;

// 1. Setup node: ./target/release/node-template --dev --tmp --rpc-methods=Unsafe --rpc-cors all
// --rpc-external --ws-external 2: ws://192.168.31.52:9944

use clap::{Command, Parser};
use cli::app::Commands;
use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::client::{Client, ClientBuilder, ClientT},
	rpc_params,
};
use rustyline::{error::ReadlineError, DefaultEditor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// TODO: Query the block number of the Polkadot
	// let addr = "127.0.0.1:9944";
	// let uri: Uri = format!("ws://{}", addr).parse()?;
	// let (tx, rx) = WsTransportClientBuilder::default().build(uri).await?;
	// let client = ClientBuilder::default().build_with_tokio(tx, rx);
	// let response: String = client.request("system_version", rpc_params![]).await?;
	// println!("bear: the result is {:?}", response);

	let mut rl = DefaultEditor::new()?;

	loop {
		let readline = rl.readline("suber >> ");
		match readline {
			Ok(line) => {
				println!("bear: --- line: {}", line);
				let command = Commands::parse_from(line.split_whitespace());
				println!("command: {:?}", command);

				match command {
					Commands::SwitchNetwork(network) => {
						todo!();
					},
					Commands::Rpc(rpc_commands) => {
						todo!()
					},
					_ => todo!(),
				}

				// rl.add_history_entry(line.as_str());
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
