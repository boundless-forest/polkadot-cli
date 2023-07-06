// std
use std::str::FromStr;
// this crate
use crate::{
	app::{AppCommand, ChainCommand, RpcCommand},
	errors::{AppError, RpcError},
	networks::ChainInfo,
	rpc::{print_format_json, ChainApi, RpcClient, SystemApi},
};

pub async fn handle_commands<CI: ChainInfo>(
	command: AppCommand,
	client: &RpcClient<CI>,
) -> Result<(), AppError> {
	match command {
		AppCommand::SwitchNetwork(_network) => {
			println!("Switch network implementation");
		},
		AppCommand::Rpc(sub_commands) => match sub_commands {
			// RpcCommand::RpcMethods => {
			// 	let res = client.rpc_methods().await?;
			// 	println!("{:?}", res);
			// },
			RpcCommand::SysName => {
				let res = client.system_name().await?;
				print_format_json(res);
			},
			RpcCommand::SysProperties => {
				let res = client.system_properties().await?;
				print_format_json(res);
			},
			RpcCommand::SysVersion => {
				let res = client.system_version().await?;
				print_format_json(res);
			},
			RpcCommand::Chain => {
				let res = client.chain().await?;
				print_format_json(res);
			},
			RpcCommand::ChainType => {
				let res = client.chain_type().await?;
				print_format_json(res);
			},
			RpcCommand::Health => {
				let res = client.health().await?;
				print_format_json(res);
			},
			RpcCommand::SyncState => {
				let res = client.sync_state().await?;
				print_format_json(res);
			},
		},
		AppCommand::Chain(sub_command) => match sub_command {
			ChainCommand::GetBlock { hash } => {
				let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
					.map_err(|_| RpcError::InvalidCommandParams)?;
				let res = client.get_block(hash).await?;
				print_format_json(res);
			},
			ChainCommand::GetBlockHash { number } => {
				let number: <CI as ChainInfo>::BlockNumber = number.into();
				let res = client.get_block_hash(number).await?;
				print_format_json(res);
			},
			ChainCommand::GetFinalizedHead => {
				let res = client.get_finalized_head().await?;
				print_format_json(res);
			},
			ChainCommand::GetHeader { hash } => {
				let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
					.map_err(|_| RpcError::InvalidCommandParams)?;
				let res = client.get_header(hash).await?;
				print_format_json(res);
			},
		},
	}

	Ok(())
}
