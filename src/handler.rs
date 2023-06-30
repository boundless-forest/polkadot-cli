use crate::command::{AppCommand, RpcCommand};
use colored::Colorize;

pub fn handle_commands(command: AppCommand) -> anyhow::Result<()> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			println!("Switch network implementation");
		},
		AppCommand::Rpc(rpc_commands) => {
			match rpc_commands {
				RpcCommand::ChainBlockByHash { hash, number } => {
					println!("ChainBlockByHash implementation, hash: {}, number: {}", hash, number);
				},
				RpcCommand::ChainHeader { hash } => {
					println!("ChainHeader implementation, hash: {}", hash);
				},
				RpcCommand::RpcMethods => {},
				RpcCommand::SysName => {},
				RpcCommand::SysProperties => {},
				RpcCommand::SysVersion => {},
				_ => {
					println!(
						"{}",
						"Invalid RPC command, please check your command and input params".red()
					);
				},
			}
			println!("Missing RPC network implementation");
		},
		_ => {
			println!("{}", "Invalid command, please check your command and input params".red());
		},
	}

	Ok(())
}
