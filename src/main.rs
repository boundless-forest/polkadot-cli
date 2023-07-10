mod app;
mod errors;
mod handler;
mod networks;
mod rpc;

// crates.io
use clap::Parser;
use colored::Colorize;
use rpc::RpcClient;
use rustyline::error::ReadlineError;
// this crate
use crate::{
	app::{load_history, print_info, this_crate_editor, AppCommand},
	errors::AppError,
	networks::{NoteTemplate, PangolinChain},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let mut editor = this_crate_editor();
	let history_file = load_history()?;
	editor.load_history(&history_file).map_err(AppError::Readline)?;

	// TODO: FIX THE URI
	// let rpc_client = RpcClient::<PangolinChain>::new("192.168.31.52:9944").await?;
	let rpc_client = RpcClient::<PangolinChain>::new("wss://pangolin-rpc.darwinia.network").await?;

	print_info();
	loop {
		// TODO: Add the network tip before >>
		let prompt = editor.readline(&"suber >> ".green().bold().to_string());
		match prompt {
			Ok(prompt) => {
				if let Ok(command) = AppCommand::try_parse_from(prompt.split_whitespace()) {
					handler::handle_commands(command, &rpc_client).await?;
				} else {
					println!("Invalid input: {}", prompt.bright_red());
				}
			},
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			},
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			},
		}
	}
	editor.save_history(&history_file)?;

	Ok(())
}
