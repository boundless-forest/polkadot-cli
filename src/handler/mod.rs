// std
use std::str::FromStr;
// crates.io
use clap::Command;
use colored::Colorize;
use frame_metadata::RuntimeMetadata;
use frame_system::AccountInfo;
use pallet_balances::AccountData;
use prettytable::{row, Table};
use serde::Serialize;
use sp_runtime::traits::Header;
// this crate
use crate::{
	app::{AccountInfoCommand, AppCommand, ChainCommand, RpcCommand, RuntimeCommand},
	errors::AppError,
	networks::{ChainInfo, Network},
	rpc::{
		single_map_storage_key, AccountBalances, AccountNonce, ChainApi, RpcClient, RpcError,
		RpcResult, StateApi, SystemApi,
	},
};

pub async fn handle_commands<CI: ChainInfo>(
	command: AppCommand,
	client: &RpcClient<CI>,
) -> Result<ExecutionResult, AppError> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			return Ok(ExecutionResult::SwitchNetworkTo(network));
		},
		AppCommand::Rpc(sub_commands) => match sub_commands {
			RpcCommand::SysName => {
				let res = client.system_name().await;
				print_result(res);
			},
			RpcCommand::SysProperties => {
				let res = client.system_properties().await;
				print_result(res);
			},
			RpcCommand::SysVersion => {
				let res = client.system_version().await;
				print_result(res);
			},
			RpcCommand::Chain => {
				let res = client.chain().await;
				print_result(res);
			},
			RpcCommand::ChainType => {
				let res = client.chain_type().await;
				print_result(res);
			},
			RpcCommand::Health => {
				let res = client.health().await;
				print_result(res);
			},
			RpcCommand::SyncState => {
				let res = client.sync_state().await;
				print_result(res);
			},
			RpcCommand::Usage => {
				print_usage::<RpcCommand>("substrate-cli rpc");
			},
		},
		AppCommand::Chain(sub_command) => match sub_command {
			ChainCommand::GetBlock { hash } => {
				let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
					.map_err(|_| RpcError::InvalidParams)?;
				let res = client.get_block(hash).await;
				print_result(res);
			},
			ChainCommand::GetBlockHash { number } => {
				let number: <CI as ChainInfo>::BlockNumber = number.into();
				let res = client.get_block_hash(number).await;
				print_result(res);
			},
			ChainCommand::GetFinalizedHead => {
				let res = client.get_finalized_head().await;
				print_result(res);
			},
			ChainCommand::GetFinalizedNumber => {
				let finalized_hash = client.get_finalized_head().await?;

				if let Some(hash) = finalized_hash {
					let res = client.get_header(hash).await?;
					print_result(Ok(res.number()));
				}
			},

			ChainCommand::GetHeader { hash } => {
				let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
					.map_err(|_| RpcError::InvalidParams)?;
				let res = client.get_header(hash).await;
				print_result(res);
			},
			ChainCommand::Usage => {
				print_usage::<ChainCommand>("substrate-cli chain");
			},
		},
		AppCommand::AccountInfo(sub_command) => match sub_command {
			AccountInfoCommand::Balances { account_id, at_block } => {
				let metadata = client.runtime_metadata().await?;
				let hash = at_block.and_then(|s| <CI as ChainInfo>::Hash::from_str(&s).ok());

				let key = <CI as ChainInfo>::AccountId::from_str(account_id.as_str())
					.map_err(|_| RpcError::InvalidParams)?;
				let storage_key = single_map_storage_key(&metadata, "System", "Account", key)
					.map_err(|_| RpcError::GenerateStorageKeyFailed)?;

				let account: Option<AccountInfo<CI::Nonce, AccountData<CI::Balance>>> =
					client.get_storage(storage_key, hash).await?;
				if let Some(a) = account {
					print_result(Ok(AccountBalances {
						free: a.data.free,
						reserved: a.data.reserved,
						frozen: a.data.frozen,
					}));
				}
			},
			AccountInfoCommand::Nonce { account_id, at_block } => {
				let metadata = client.runtime_metadata().await?;
				let hash = at_block.and_then(|s| <CI as ChainInfo>::Hash::from_str(&s).ok());

				let key = <CI as ChainInfo>::AccountId::from_str(account_id.as_str())
					.map_err(|_| RpcError::InvalidParams)?;
				let storage_key = single_map_storage_key(&metadata, "System", "Account", key)
					.map_err(|_| RpcError::GenerateStorageKeyFailed)?;

				let account: Option<AccountInfo<CI::Nonce, AccountData<CI::Balance>>> =
					client.get_storage(storage_key, hash).await?;
				if let Some(a) = account {
					print_result(Ok(AccountNonce { nonce: a.nonce }));
				}
			},
			AccountInfoCommand::Usage => {
				print_usage::<AccountInfoCommand>("substrate-cli account-info");
			},
		},
		AppCommand::Runtime(sub_command) => match sub_command {
			RuntimeCommand::ListPallets => {
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
			RuntimeCommand::ListPalletStorages { pallet_name } => {
				let metadata = client.runtime_metadata().await?;
				let RuntimeMetadata::V14(metadata) = &metadata.1  else {
					return Err(AppError::Custom("Only support the runtime metadata V14 now.".to_string()));
				};

				match metadata.pallets.iter().find(|p| p.name == pallet_name) {
					Some(p) =>
						if let Some(storage) = &p.storage {
							let mut table = Table::new();
							table.add_row(row!["Storage Name", "Doc"]);
							storage.entries.iter().for_each(|e| {
								table.add_row(row![
									e.name.bold(),
									e.docs.get(0).unwrap_or(&"".to_owned())
								]);
							});
							table.printstd();
						},
					None => {
						println!("Did not find the pallet.");
					},
				};
			},
			RuntimeCommand::ListPalletConstants { pallet_name } => {
				let metadata = client.runtime_metadata().await?;
				let RuntimeMetadata::V14(metadata) = &metadata.1  else {
					return Err(AppError::Custom("Only support the runtime metadata V14 now.".to_string()));
				};

				match metadata.pallets.iter().find(|p| p.name == pallet_name) {
					Some(p) => {
						let mut table = Table::new();
						table.add_row(row!["Constant Name", "Doc"]);
						p.constants.iter().for_each(|c| {
							table.add_row(row![
								c.name.bold(),
								c.docs.get(0).unwrap_or(&"".to_owned())
							]);
						});
						table.printstd();
					},
					None => {
						println!("Did not find the pallet.");
					},
				};
			},
			RuntimeCommand::RuntimeVersion { hash } => {
				let hash = if let Some(hash) = hash {
					<CI as ChainInfo>::Hash::from_str(hash.as_str())
						.map_err(|_| RpcError::InvalidParams)?
				} else {
					client.get_finalized_head().await?.expect("Failed to get finalized head")
				};

				let res = client.runtime_version(hash).await;
				print_result(res);
			},
			RuntimeCommand::Usage => {
				print_usage::<RuntimeCommand>("substrate-cli runtime");
			},
		},
		AppCommand::Usage => {
			print_usage::<AppCommand>("substrate-cli");
		},
	}

	Ok(ExecutionResult::Success)
}

/// The APP's command execution result.
pub enum ExecutionResult {
	/// Switch to another network.
	SwitchNetworkTo(Network),
	/// Execute successfully.
	Success,
	/// Execute failed.
	Exited,
}

/// Print the result in JSON format.
pub fn print_result<T: Serialize>(data: RpcResult<T>) {
	let Ok(data) = data else {
		println!("{}", RpcError::EmptyResult.to_string().italic().bright_magenta());
		return;
	};

	if let Ok(data) = serde_json::to_string_pretty(&data) {
		println!("{}", data.italic().bright_magenta());
	} else {
		println!("{}", RpcError::InvalidJsonObject.to_string().italic().bright_magenta());
	}
}

pub fn print_usage<T: clap::Subcommand>(command_name: &'static str) {
	let mock = Command::new(command_name)
		.disable_help_flag(true)
		.disable_help_subcommand(true)
		.no_binary_name(true);
	let mut command = <T as clap::Subcommand>::augment_subcommands(mock);
	println!("{}", command.render_long_help());
}
