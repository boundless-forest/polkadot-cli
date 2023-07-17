mod app;
mod errors;
mod handler;
mod networks;
mod rpc;

use app::Network;
// crates.io
use clap::Parser;
use colored::Colorize;
use networks::{ChainInfo, CrabChain, DarwiniaChain, PangoroChain};
use rpc::RpcClient;
use rustyline::{error::ReadlineError, hint::HistoryHinter, history::FileHistory, Editor};
// this crate
use crate::{
	app::{load_history, print_info, this_crate_editor, AppCommand, CommandHelper},
	errors::{AppError, RpcError},
	networks::{NoteTemplate, PangolinChain},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let mut editor = this_crate_editor();
	let history_file = load_history()?;
	editor.load_history(&history_file).map_err(AppError::Readline)?;

	// TODO: fix the unwrap()
	let config = editor.helper().unwrap().config();
	match config.network {
		Network::Local => {
			let rpc_client = RpcClient::<NoteTemplate>::new(config).await?;
			run(&mut editor, &rpc_client).await?
		},
		Network::Polkadot => {
			// let rpc_client = RpcClient::<>::new(config).await?;
			todo!();
		},
		Network::Kusama => {
			// let rpc_client = RpcClient::<NoteTemplate>::new(config).await?;
			todo!();
		},
		Network::Crab => {
			let rpc_client = RpcClient::<CrabChain>::new(config).await?;
			run(&mut editor, &rpc_client).await?
		},
		Network::Darwinia => {
			let rpc_client = RpcClient::<DarwiniaChain>::new(config).await?;
			run(&mut editor, &rpc_client).await?
		},
		Network::Pangolin => {
			let rpc_client = RpcClient::<PangolinChain>::new(config).await?;
			run(&mut editor, &rpc_client).await?
		},
		Network::Pangoro => {
			let rpc_client = RpcClient::<PangoroChain>::new(config).await?;
			run(&mut editor, &rpc_client).await?
		},
	}

	editor.save_history(&history_file)?;
	Ok(())
}

pub async fn run<CI: ChainInfo>(
	editor: &mut Editor<CommandHelper<HistoryHinter>, FileHistory>,
	rpc_client: &RpcClient<CI>,
) -> Result<(), AppError> {
	print_info();
	loop {
		let command_tip = format!("suber ({:?}) >> ", <CI as ChainInfo>::NET_WORK);
		let prompt = editor.readline(&command_tip.bright_yellow().italic().bold().to_string());
		match prompt {
			Ok(prompt) => {
				if let Ok(command) = AppCommand::try_parse_from(prompt.split_whitespace()) {
					handler::handle_commands(command, &rpc_client).await?;
				} else {
					continue;
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
	Ok(())
}
