// crates.io
use colored::Colorize;
// this crate
use crate::{
	app::{AppCommand, RpcCommand},
	errors::{AppError, HandlerError},
	rpc::{format_json_output, RpcClient, SystemApi},
};

pub async fn handle_commands(command: AppCommand, client: &RpcClient) -> Result<(), AppError> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			println!("Switch network implementation");
		},
		AppCommand::Rpc(rpc_commands) => match rpc_commands {
			// RpcCommand::RpcMethods => {
			// 	let res = client.rpc_methods().await?;
			// 	println!("{:?}", res);
			// },
			RpcCommand::SysName => {
				let res = client.system_name().await?;
				println!("{:?}", res);
			},
			RpcCommand::SysProperties => {
				let res = client.system_properties().await?;
				println!("{}", format_json_output(res)?);
			},
			RpcCommand::SysVersion => {
				let res = client.system_version().await?;
				println!("{:?}", res);
			},
			RpcCommand::Chain => {
				let res = client.chain().await?;
				println!("{:?}", res);
			},
			RpcCommand::ChainType => {
				let res = client.chain_type().await?;
				println!("{}", format_json_output(res)?);
			},
			RpcCommand::Health => {
				let res = client.health().await?;
				println!("{}", format_json_output(res)?);
			},
			RpcCommand::SyncState => {
				let res = client.sync_state().await?;
				println!("{}", format_json_output(res)?);
			},
			RpcCommand::ChainBlockByHash { hash, number } => {
				println!("ChainBlockByHash implementation, hash: {}, number: {}", hash, number);
			},
			RpcCommand::ChainHeader { hash } => {
				println!("ChainHeader implementation, hash: {}", hash);
			},
			_ => {
				println!(
					"{}",
					"Invalid RPC command, please check your command and input params".red()
				);
			},
		},
		_ => {
			eprintln!("{}", "Invalid command, please check your command and input params".red());
			return Err(HandlerError::UnknownAppCommand.into());
		},
	}

	Ok(())
}
