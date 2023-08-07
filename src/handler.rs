// std
use std::str::FromStr;
// crates.io
use frame_metadata::RuntimeMetadata;
use frame_system::AccountInfo;
use pallet_balances::AccountData;
use prettytable::{row, Table};
use sp_runtime::traits::Header;
// this crate
use crate::{
	app::{AccountInfoCommand, AppCommand, ChainCommand, PalletsCommand, RpcCommand, StateCommand},
	errors::{AppError, RpcError},
	networks::{ChainInfo, Network},
	rpc::{
		print_format_json, single_map_storage_key, AccountBalances, AccountNonce, ChainApi,
		RpcClient, StateApi, SystemApi,
	},
};

/// The APP's command execution result.
pub enum ExecutionResult {
	/// Switch to another network.
	SwitchNetworkTo(Network),
	/// Execute successfully.
	Success,
	/// Execute failed.
	Exited,
}

pub async fn handle_commands<CI: ChainInfo>(
	command: AppCommand,
	client: &RpcClient<CI>,
) -> Result<ExecutionResult, AppError> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			return Ok(ExecutionResult::SwitchNetworkTo(network));
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
			ChainCommand::GetFinalizedBlockNumber => {
				let finalized_hash = client.get_finalized_head().await?;
				if let Some(hash) = finalized_hash {
					let res = client.get_header(hash).await?;
					print_format_json(res.number());
				} else {
					print_format_json(None::<String>);
				}
			},

			ChainCommand::GetHeader { hash } => {
				let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
					.map_err(|_| RpcError::InvalidCommandParams)?;
				let res = client.get_header(hash).await?;
				print_format_json(res);
			},
		},
		AppCommand::State(sub_command) => match sub_command {
			StateCommand::RuntimeVersion { hash } => {
				let hash = if let Some(hash) = hash {
					<CI as ChainInfo>::Hash::from_str(hash.as_str())
						.map_err(|_| RpcError::InvalidCommandParams)?
				} else {
					client.get_finalized_head().await?.expect("Failed to get finalized head")
				};

				let res = client.runtime_version(hash).await?;
				print_format_json(res);
			},
		},
		AppCommand::AccountInfo(sub_command) => match sub_command {
			AccountInfoCommand::Balances { account_id, at_block } => {
				let metadata = client.runtime_metadata().await?;
				let hash = at_block.and_then(|s| <CI as ChainInfo>::Hash::from_str(&s).ok());

				let key = <CI as ChainInfo>::AccountId::from_str(account_id.as_str())
					.map_err(|_| RpcError::InvalidCommandParams)?;
				let storage_key = single_map_storage_key(&metadata, "System", "Account", key)
					.map_err(|_| RpcError::StorageKeyFailed)?;

				let account: Option<AccountInfo<CI::Nonce, AccountData<CI::Balance>>> =
					client.get_storage(storage_key, hash).await?;
				if let Some(a) = account {
					print_format_json(AccountBalances {
						free: a.data.free,
						reserved: a.data.reserved,
						frozen: a.data.frozen,
					});
				}
			},
			AccountInfoCommand::Nonce { account_id, at_block } => {
				let metadata = client.runtime_metadata().await?;
				let hash = at_block.and_then(|s| <CI as ChainInfo>::Hash::from_str(&s).ok());

				let key = <CI as ChainInfo>::AccountId::from_str(account_id.as_str())
					.map_err(|_| RpcError::InvalidCommandParams)?;
				let storage_key = single_map_storage_key(&metadata, "System", "Account", key)
					.map_err(|_| RpcError::StorageKeyFailed)?;

				let account: Option<AccountInfo<CI::Nonce, AccountData<CI::Balance>>> =
					client.get_storage(storage_key, hash).await?;
				if let Some(a) = account {
					print_format_json(AccountNonce { nonce: a.nonce });
				}
			},
		},
		AppCommand::Pallets(sub_command) => match sub_command {
			PalletsCommand::ListAll => {
				let metadata = client.runtime_metadata().await?;
				let RuntimeMetadata::V14(metadata) = &metadata.1  else {
					return Err(AppError::Custom("Only support the runtime metadata V14 now.".to_string()));
				};

				let mut pallets = metadata.pallets.to_vec();
				pallets.sort_by_key(|p| p.index);

				let mut table = Table::new();
				table.add_row(row!["Pallet", "Index"]);
				pallets.iter().for_each(|p| {
					table.add_row(row![p.name, p.index]);
				});
				table.printstd();
			},
		},
	}

	Ok(ExecutionResult::Success)
}
