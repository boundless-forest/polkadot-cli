mod command;
mod errors;
mod handler;
mod helper;
mod rpc;

// 1. Setup node: ./target/release/node-template --dev --tmp --rpc-methods=Unsafe --rpc-cors all
// --rpc-external --ws-external 2: ws://192.168.31.52:9944

use clap::Parser;
use colored::Colorize;
use helper::{load_history, print_info, this_crate_editor};
use rpc::{RpcClient, RpcError, RpcResult};
use rustyline::{
	error::ReadlineError,
	history::{self},
};

use crate::{command::AppCommand, errors::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let mut editor = this_crate_editor();
	let history_file = load_history()?;
	editor.load_history(&history_file).map_err(AppError::Readline)?;

	let rpc_client = RpcClient::new("192.168.31.52:9944").await?;

	print_info();
	loop {
		let prompt = editor.readline(&"suber >> ".green().bold().to_string());
		match prompt {
			Ok(prompt) => {
				if let Ok(command) = AppCommand::try_parse_from(prompt.split_whitespace()) {
					handler::handle_commands(command)?;
				} else {
					println!("Invalid prompt: {}", prompt);
				}
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
	editor.save_history(&history_file)?;

	Ok(())
}
