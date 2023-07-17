mod app;
mod errors;
mod handler;
mod networks;
mod rpc;

// crates.io
use clap::Parser;
use colored::Colorize;
use handler::HandleResult;
use networks::{ChainInfo, CrabChain, DarwiniaChain, Network, PangoroChain};
use rpc::RpcClient;
use rustyline::{error::ReadlineError, hint::HistoryHinter, history::FileHistory, Editor};
// this crate
use crate::{
	app::{load_history, print_info, this_crate_editor, AppCommand, CommandHelper, Config},
	errors::AppError,
	networks::{NoteTemplate, PangolinChain},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let mut editor = this_crate_editor();
	let history_file = load_history()?;
	editor.load_history(&history_file).map_err(AppError::Readline)?;

	loop {
		let config = editor.helper_mut().unwrap().load_config().unwrap();
		match config.network {
			Network::Local => {
				let rpc_client = RpcClient::<NoteTemplate>::new().await?;
				if let Ok(HandleResult::SwitchNetworkTo(network)) =
					run(&mut editor, &rpc_client).await
				{
					editor.helper_mut().unwrap().save_config(Config { network })?;
					editor.save_history(&history_file)?;
					continue;
				}
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
				let rpc_client = RpcClient::<CrabChain>::new().await?;
				if let Ok(HandleResult::SwitchNetworkTo(network)) =
					run(&mut editor, &rpc_client).await
				{
					editor.helper_mut().unwrap().save_config(Config { network })?;
					editor.save_history(&history_file)?;
					continue;
				}
			},
			Network::Darwinia => {
				let rpc_client = RpcClient::<DarwiniaChain>::new().await?;
				if let Ok(HandleResult::SwitchNetworkTo(network)) =
					run(&mut editor, &rpc_client).await
				{
					editor.helper_mut().unwrap().save_config(Config { network })?;
					editor.save_history(&history_file)?;
					continue;
				}
			},
			Network::Pangolin => {
				let rpc_client = RpcClient::<PangolinChain>::new().await?;
				if let Ok(HandleResult::SwitchNetworkTo(network)) =
					run(&mut editor, &rpc_client).await
				{
					editor.helper_mut().unwrap().save_config(Config { network })?;
					editor.save_history(&history_file)?;
					continue;
				}
			},
			Network::Pangoro => {
				let rpc_client = RpcClient::<PangoroChain>::new().await?;
				if let Ok(HandleResult::SwitchNetworkTo(network)) =
					run(&mut editor, &rpc_client).await
				{
					editor.helper_mut().unwrap().save_config(Config { network })?;
					editor.save_history(&history_file)?;
					continue;
				}
			},
		}
	}
}

pub async fn run<CI: ChainInfo>(
	editor: &mut Editor<CommandHelper<HistoryHinter>, FileHistory>,
	rpc_client: &RpcClient<CI>,
) -> Result<HandleResult, AppError> {
	print_info();
	loop {
		let command_tip = format!("suber ({:?}) >> ", <CI as ChainInfo>::NET_WORK);
		let prompt = editor.readline(&command_tip.bright_yellow().italic().bold().to_string());
		match prompt {
			Ok(prompt) => {
				if let Ok(command) = AppCommand::try_parse_from(prompt.split_whitespace()) {
					if let Ok(HandleResult::SwitchNetworkTo(network)) =
						handler::handle_commands(command, &rpc_client).await
					{
						return Ok(HandleResult::SwitchNetworkTo(network));
					}
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
	Ok(HandleResult::Success)
}
