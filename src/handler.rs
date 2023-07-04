// crates.io
use colored::Colorize;
// this crate
use crate::{
	command::{AppCommand, RpcCommand},
	errors::{AppError, HandlerError},
	rpc::{Api, RpcClient},
};

pub async fn handle_commands(command: AppCommand, client: &RpcClient) -> Result<(), AppError> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			println!("Switch network implementation");
		},
		AppCommand::Rpc(rpc_commands) => match rpc_commands {
			RpcCommand::ChainBlockByHash { hash, number } => {
				println!("ChainBlockByHash implementation, hash: {}, number: {}", hash, number);
			},
			RpcCommand::ChainHeader { hash } => {
				println!("ChainHeader implementation, hash: {}", hash);
			},
			RpcCommand::RpcMethods => {
				let res = client.rpc_methods().await?;
				println!("{:?}", res);
			},
			RpcCommand::SysName => {
				let res = client.system_name().await?;
				println!("{:?}", res);
			},
			RpcCommand::SysProperties => {
				let res = client.system_properties().await?;
				println!("{:?}", res);
			},
			RpcCommand::SysVersion => {
				let res = client.system_version().await?;
				println!("{:?}", res);
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
