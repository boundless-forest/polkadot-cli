mod app;
mod errors;
mod handler;
mod networks;
mod rpc;

// crates.io
use clap::Parser;
use colored::Colorize;
use handler::ExecutionResult;
use networks::{
	ChainInfo, CrabChain, DarwiniaChain, KusamaChain, Network, PangoroChain, PolkadotChain,
};
use rpc::RpcClient;
use rustyline::{hint::HistoryHinter, history::FileHistory, Editor};
// this crate
use crate::{
	app::{create_editor, history_path, print_welcome_message, AppCommand, Config, EditorHelper},
	errors::AppError,
	networks::{NoteTemplate, PangolinChain},
};

macro_rules! switch_network_or_break {
	($editor: expr, $rpc_client: expr) => {
		match run(&mut $editor, &$rpc_client).await {
			Ok(ExecutionResult::SwitchNetworkTo(network)) => {
				$editor.helper_mut().unwrap().save_config(Config { network })?;
				continue;
			},
			_ => {
				break;
			},
		}
	};
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
	env_logger::init();
	let mut editor = create_editor();
	let path = history_path().map_err(|e| AppError::Custom(format!("path err: {:?}", e)))?;
	editor
		.load_history(&path)
		.map_err(|_| AppError::Custom("Failed to load history".to_string()))?;

	print_welcome_message();
	loop {
		let config = editor.helper_mut().unwrap().load_config().unwrap();
		editor.save_history(&path)?;

		log::debug!(target: "cli", "Starting up, network: {:?}", config.network);
		match config.network {
			Network::Local => {
				let rpc_client = RpcClient::<NoteTemplate>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
			Network::Polkadot => {
				let rpc_client = RpcClient::<PolkadotChain>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
			Network::Kusama => {
				let rpc_client = RpcClient::<KusamaChain>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
			Network::Crab => {
				let rpc_client = RpcClient::<CrabChain>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
			Network::Darwinia => {
				let rpc_client = RpcClient::<DarwiniaChain>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
			Network::Pangolin => {
				let rpc_client = RpcClient::<PangolinChain>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
			Network::Pangoro => {
				let rpc_client = RpcClient::<PangoroChain>::new().await?;
				switch_network_or_break!(&mut editor, &rpc_client);
			},
		}
	}
	Ok(())
}

// Command execution loop
pub async fn run<CI: ChainInfo>(
	editor: &mut Editor<EditorHelper<HistoryHinter>, FileHistory>,
	rpc_client: &RpcClient<CI>,
) -> Result<ExecutionResult, AppError> {
	loop {
		let command_tip = format!("substrate-cli ({:?}) >> ", <CI as ChainInfo>::NET_WORK)
			.bright_green()
			.italic();
		let prompt = editor.readline(command_tip.to_string().as_str());
		match prompt {
			Ok(prompt) => match AppCommand::try_parse_from(prompt.split_whitespace()) {
				Ok(command) => {
					log::debug!(target: "cli", "command: {:?}", command);
					if let Ok(ExecutionResult::SwitchNetworkTo(network)) =
						handler::handle_commands(command, rpc_client).await
					{
						return Ok(ExecutionResult::SwitchNetworkTo(network));
					}
				},
				Err(e) => {
					log::debug!(target: "cli", "failed to parse the command, err: {:?}", e);
					println!(
						"{}",
						"invalid input, TAB to complete.".to_string().italic().bright_red()
					);
					continue;
				},
			},
			_ => return Ok(ExecutionResult::Exited),
		}
	}
}
