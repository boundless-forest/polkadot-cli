mod printer;

// std
use std::{
	fs::File,
	io::{Read, Write},
	str::FromStr,
};
// crates.io
use colored::Colorize;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use frame_system::AccountInfo;
use pallet_balances::AccountData;
use sp_core::{Decode, Encode};
use sp_runtime::traits::Header;
use subxt_metadata::{Metadata, PalletMetadata};
// this crate
use crate::{
	app::{
		metadata_path, AccountInfoCommand, AppCommand, ApplicationCommand, ChainCommand,
		RpcCommand, RuntimeCommand, POLKADOT_CLI,
	},
	errors::AppError,
	handler::printer::{print_result, print_storage_type, print_usage},
	networks::{ChainInfo, Network},
	rpc::{
		single_map_storage_key, AccountBalances, AccountNonce, ChainApi, RpcClient, RpcError,
		StateApi, SystemApi,
	},
};

pub struct Handler<'a, CI> {
	client: &'a RpcClient<CI>,
	metadata: Metadata,
}

impl<'a, CI: ChainInfo> Handler<'a, CI> {
	/// Create a new handler
	pub async fn new(client: &'a RpcClient<CI>) -> Result<Handler<'a, CI>, AppError> {
		let metadata_path = metadata_path().expect("Failed to get metadata path");
		let runtime_version = client
			.runtime_version(
				client.get_finalized_head().await?.expect("Failed to get finalized head"),
			)
			.await
			.expect("Failed to get runtime version");
		let file_name =
			format!("{}-runtime-{}.meta", runtime_version.spec_name, runtime_version.spec_version);
		let runtime_file = metadata_path.join(file_name);
		log::debug!(target: "cli", "Runtime metadata file path: {:?}", runtime_file);

		if !runtime_file.is_file() {
			// New network or new runtime version detected.
			let metadata = client.runtime_metadata().await?;
			let mut metadata_file = File::create(runtime_file).map_err(|e| {
				AppError::Custom(format!("Failed to create metadata file: {:?}", e))
			})?;
			metadata_file
				.write_all(&metadata.encode())
				.map_err(|e| AppError::Custom(format!("Failed to write metadata file: {:?}", e)))?;
			log::debug!(target: "cli", "Wrote runtime metadata file successfully");
			Ok(Self { client, metadata })
		} else {
			// Reload from the existed runtime metadata file.
			let mut f = File::open(runtime_file).map_err(|e| {
				AppError::Custom(format!("Failed to open runtime metadata file: {:?}", e))
			})?;
			let mut bytes = Vec::new();
			f.read_to_end(&mut bytes).map_err(|e| {
				AppError::Custom(format!("Failed to read runtime metadata file: {:?}", e))
			})?;
			let metadata = Metadata::decode(&mut bytes.as_slice()).map_err(|e| {
				AppError::Custom(format!("Failed to decode runtime metadata file: {:?}", e))
			})?;
			log::debug!(target: "cli", "Reload runtime metadata file successfully");
			Ok(Self { client, metadata })
		}
	}

	/// Execute the captured command
	pub async fn handle_command(&self, command: AppCommand) -> Result<ExecutionResult, AppError> {
		match command {
			AppCommand::App(sub_commands) => match sub_commands {
				ApplicationCommand::SwitchNetwork { network } => {
					return Ok(ExecutionResult::SwitchNetworkTo(network));
				},
				ApplicationCommand::DashBoard => {
					todo!();
				},
				ApplicationCommand::CleanHistory => {
					todo!();
				},
				ApplicationCommand::Usage => {
					todo!();
				},
			},
			AppCommand::Rpc(sub_commands) => match sub_commands {
				RpcCommand::SysName => {
					let res = self.client.system_name().await;
					print_result(res);
				},
				RpcCommand::SysProperties => {
					let res = self.client.system_properties().await;
					print_result(res);
				},
				RpcCommand::SysVersion => {
					let res = self.client.system_version().await;
					print_result(res);
				},
				RpcCommand::Chain => {
					let res = self.client.chain().await;
					print_result(res);
				},
				RpcCommand::ChainType => {
					let res = self.client.chain_type().await;
					print_result(res);
				},
				RpcCommand::Health => {
					let res = self.client.health().await;
					print_result(res);
				},
				RpcCommand::SyncState => {
					let res = self.client.sync_state().await;
					print_result(res);
				},
				RpcCommand::Usage => {
					let command_name = format!("{} rpc", POLKADOT_CLI);
					print_usage::<RpcCommand>(command_name.into());
				},
			},
			AppCommand::Chain(sub_command) => match sub_command {
				ChainCommand::GetBlock { hash } => {
					let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
						.map_err(|_| RpcError::InvalidParams)?;
					let res = self.client.get_block(hash).await;
					print_result(res);
				},
				ChainCommand::GetBlockHash { number } => {
					let number: <CI as ChainInfo>::BlockNumber = number.into();
					let res = self.client.get_block_hash(number).await;
					print_result(res);
				},
				ChainCommand::GetFinalizedHead => {
					let res = self.client.get_finalized_head().await;
					print_result(res);
				},
				ChainCommand::GetFinalizedNumber => {
					let finalized_hash = self.client.get_finalized_head().await?;

					if let Some(hash) = finalized_hash {
						let res = self.client.get_header(hash).await?;
						print_result(Ok(res.number()));
					}
				},
				ChainCommand::GetHeader { hash } => {
					let hash = <CI as ChainInfo>::Hash::from_str(hash.as_str())
						.map_err(|_| RpcError::InvalidParams)?;
					let res = self.client.get_header(hash).await;
					print_result(res);
				},
				ChainCommand::Usage => {
					let command_name = format!("{} chain", POLKADOT_CLI);
					print_usage::<ChainCommand>(command_name.into());
				},
			},
			AppCommand::AccountInfo(sub_command) => match sub_command {
				AccountInfoCommand::Balances { account_id, at_block } => {
					let hash = at_block.and_then(|s| <CI as ChainInfo>::Hash::from_str(&s).ok());

					let key = <CI as ChainInfo>::AccountId::from_str(account_id.as_str())
						.map_err(|_| RpcError::InvalidParams)?;
					let storage_key =
						single_map_storage_key(&self.metadata, "System", "Account", key)
							.map_err(|_| RpcError::GenerateStorageKeyFailed)?;

					let account: Option<AccountInfo<CI::Nonce, AccountData<CI::Balance>>> =
						self.client.get_storage(storage_key, hash).await?;
					if let Some(a) = account {
						print_result(Ok(AccountBalances {
							free: a.data.free,
							reserved: a.data.reserved,
							frozen: a.data.frozen,
						}));
					}
				},
				AccountInfoCommand::Nonce { account_id, at_block } => {
					let hash = at_block.and_then(|s| <CI as ChainInfo>::Hash::from_str(&s).ok());

					let key = <CI as ChainInfo>::AccountId::from_str(account_id.as_str())
						.map_err(|_| RpcError::InvalidParams)?;
					let storage_key =
						single_map_storage_key(&self.metadata, "System", "Account", key)
							.map_err(|_| RpcError::GenerateStorageKeyFailed)?;

					let account: Option<AccountInfo<CI::Nonce, AccountData<CI::Balance>>> =
						self.client.get_storage(storage_key, hash).await?;
					if let Some(a) = account {
						print_result(Ok(AccountNonce { nonce: a.nonce }));
					}
				},
				AccountInfoCommand::Usage => {
					let command_name = format!("{} account-info", POLKADOT_CLI);
					print_usage::<AccountInfoCommand>(command_name.into());
				},
			},
			AppCommand::Runtime(sub_command) => match sub_command {
				RuntimeCommand::ListAllPallets => {
					let mut table = Table::new();
					table
						.set_header(vec!["Pallet", "Index"])
						.load_preset(UTF8_FULL)
						.apply_modifier(UTF8_ROUND_CORNERS)
						.set_content_arrangement(ContentArrangement::Dynamic);
					self.metadata.pallets().for_each(|p| {
						table.add_row(vec![p.name(), &p.index().to_string()]);
					});
					println!("{table}");
				},
				RuntimeCommand::StoragesOfPallet { pallet_name, pallet_id } => {
					let pallet: Option<PalletMetadata> = match (pallet_name, pallet_id) {
						(Some(name), Some(id)) =>
							self.metadata.pallets().find(|p| p.name() == name && p.index() == id),
						(Some(name), None) => self.metadata.pallet_by_name(&name),
						(None, Some(id)) => self.metadata.pallet_by_index(id),
						_ => None,
					};
					log::debug!(target: "cli", "Fetch the pallet metadata result: {:?}", pallet.is_some());

					if let Some(p) = pallet {
						if let Some(s) = p.storage() {
							let mut table = Table::new();
							table
								.set_header(vec!["NAME", "TYPE", "DOC"])
								.load_preset(UTF8_FULL)
								.apply_modifier(UTF8_ROUND_CORNERS)
								.set_content_arrangement(ContentArrangement::Dynamic);
							s.entries().iter().for_each(|e| {
								table.add_row(vec![
									e.name(),
									&print_storage_type(e.entry_type().clone(), &self.metadata),
									e.docs().get(0).unwrap_or(&"".to_owned()),
								]);
							});

							println!(
								"PalletName: {}, PalletIndex: {}",
								p.name().red().bold(),
								p.index().to_string().red().bold()
							);
							println!("{table}");
						}
					} else {
						println!("Did not find the pallet.");
					}
				},
				RuntimeCommand::ConstantsOfPallet { pallet_name, pallet_id } => {
					let pallet: Option<PalletMetadata> = match (pallet_name, pallet_id) {
						(Some(name), Some(id)) =>
							self.metadata.pallets().find(|p| p.name() == name && p.index() == id),
						(Some(name), None) => self.metadata.pallet_by_name(&name),
						(None, Some(id)) => self.metadata.pallet_by_index(id),
						_ => None,
					};
					log::debug!(target: "cli", "Fetch the pallet metadata result: {:?}", pallet.is_some());

					if let Some(p) = pallet {
						let mut table = Table::new();
						table
							.set_header(vec!["NAME", "DOC"])
							.load_preset(UTF8_FULL)
							.apply_modifier(UTF8_ROUND_CORNERS)
							.set_content_arrangement(ContentArrangement::Dynamic);
						p.constants().for_each(|c| {
							table
								.add_row(vec![c.name(), c.docs().get(0).unwrap_or(&"".to_owned())]);
						});

						println!(
							"PalletName: {}, PalletIndex: {}",
							p.name().red().bold(),
							p.index().to_string().red().bold()
						);
						println!("{table}");
					} else {
						println!("Did not find the pallet.");
					}
				},
				RuntimeCommand::GetConstantByName { pallet_name, pallet_id, constant_name } => {
					let pallet: Option<PalletMetadata> = match (pallet_name, pallet_id) {
						(Some(name), Some(id)) =>
							self.metadata.pallets().find(|p| p.name() == name && p.index() == id),
						(Some(name), None) => self.metadata.pallet_by_name(&name),
						(None, Some(id)) => self.metadata.pallet_by_index(id),
						_ => None,
					};
					log::debug!(target: "cli", "Fetch the pallet metadata result: {:?}", pallet.is_some());

					if let Some(p) = pallet {
						if let Some(c) = p.constants().find(|c| c.name() == constant_name) {
							let ty_id = c.ty();
							let mut bytes = c.value();
							let value = scale_value::scale::decode_as_type(
								&mut bytes,
								ty_id,
								self.metadata.types(),
							)
							.map_err(|e| {
								AppError::Custom(format!(
									"Failed to decode constant value: {:?}",
									e
								))
							})?;
							println!(
								"{} => {}",
								c.name(),
								serde_json::to_string_pretty(&value).unwrap()
							);
						} else {
							println!("Did not find the constant.");
						}
					} else {
						println!("Did not find the pallet.");
					}
				},
				RuntimeCommand::GetRuntimeVersion { hash } => {
					let hash = if let Some(hash) = hash {
						<CI as ChainInfo>::Hash::from_str(hash.as_str())
							.map_err(|_| RpcError::InvalidParams)?
					} else {
						self.client
							.get_finalized_head()
							.await?
							.expect("Failed to get finalized head")
					};

					let res = self.client.runtime_version(hash).await;
					print_result(res);
				},
				RuntimeCommand::Usage => {
					let command_name = format!("{} runtime", POLKADOT_CLI);
					print_usage::<RuntimeCommand>(command_name.into());
				},
			},
			AppCommand::Usage => {
				print_usage::<AppCommand>(POLKADOT_CLI.into());
			},
		}

		Ok(ExecutionResult::Success)
	}
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
