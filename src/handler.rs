// std
use std::str::FromStr;
// crates.io
use frame_metadata::{
	v14::{StorageEntryType, StorageHasher},
	RuntimeMetadata,
};
use pallet_balances::AccountData;
// this crate
use crate::{
	app::{AccountCommand, AppCommand, ChainCommand, RpcCommand, StateCommand},
	errors::{AppError, RpcError},
	networks::{ChainInfo, Network},
	rpc::{print_format_json, ChainApi, RpcClient, StateApi, SystemApi},
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
		AppCommand::Account(sub_command) => match sub_command {
			AccountCommand::Balances { account_id, at_block } => {
				const pallet_name: &'static str = "System";
				const storage_name: &'static str = "Account";
				// TODO: FIX ME
				let encoded_key = account_id.encode();

				let module_prefix = pallet_name.as_bytes().to_vec();
				let storage_prefix = storage_name.as_bytes().to_vec();
				let mut storage_key = sp_core::twox_128(&module_prefix).to_vec();
				storage_key.extend(&sp_core::twox_128(&storage_prefix)[..]);

				let hash = if let Some(hash) = at_block {
					Some(
						<CI as ChainInfo>::Hash::from_str(hash.as_str())
							.map_err(|_| RpcError::InvalidCommandParams)?,
					)
				} else {
					None
				};

				let metadata = client.runtime_metadata().await?;
				match metadata.1 {
					RuntimeMetadata::V14(metadata) => {
						if let Some(p) =
							metadata.pallets.into_iter().find(|p| p.name == pallet_name)
						{
							if let Some(entry_metadata) =
								p.storage.map(|s| s.entries).and_then(|entries| {
									entries.clone().into_iter().find(|e| e.name == storage_name)
								}) {
								match entry_metadata.ty {
									StorageEntryType::Map { hashers, key, value } => {
										let hasher = hashers.get(0).expect("Failed to get hasher");
										match hasher {
											StorageHasher::Blake2_128Concat => {
												let x: &[u8] = encoded_key.as_slice();
												let key = sp_core::blake2_128(x)
													.iter()
													.chain(x.iter())
													.cloned()
													.collect::<Vec<_>>();
												storage_key.extend(key);
											},
											_ => {
												todo!();
											},
										}

										todo!()
									},
									_ => todo!(),
								}
							}
						}
					},
					_ => {
						todo!();
					},
				}

				let res: Option<AccountData<CI::Balance>> =
					client.get_storage(sp_storage::StorageKey(storage_key), hash).await?;
				print_format_json(res);
			},
		},
	}

	Ok(ExecutionResult::Success)
}
