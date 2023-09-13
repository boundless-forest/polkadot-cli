// crates.io
use clap::{builder::Str, Command};
use colored::Colorize;
use core::any::TypeId;
use scale_info::{interner::UntrackedSymbol, TypeDef};
use serde::Serialize;
use subxt_metadata::{Metadata, StorageEntryType};
// this crate
use crate::rpc::{RpcError, RpcResult};

/// Fetch the storage type string.
pub fn print_storage_type(entry_type: StorageEntryType, metadata: &Metadata) -> String {
	fn type_name(sym: UntrackedSymbol<TypeId>, metadata: &Metadata) -> String {
		match metadata.types().resolve(sym.id) {
			Some(ty) => {
				if let Some(i) = ty.path.ident() {
					log::debug!(target: "cli", "ident: {:?}", i);
					format!("{}", i)
				} else {
					// Need to check the real type
					match ty.type_def.clone() {
						TypeDef::Primitive(t) => {
							log::debug!(target: "cli", "Primitive: {:?}", t);
							format!("{:?}", t)
						},
						TypeDef::Array(t) => {
							format!("[{}; {}]", type_name(t.type_param, metadata), t.len)
						},
						TypeDef::Sequence(t) => {
							format!("Vec<{}>", type_name(t.type_param, metadata))
						},
						TypeDef::Tuple(t) => {
							format!(
								"({})",
								t.fields
									.clone()
									.into_iter()
									.enumerate()
									.map(|(i, f)| {
										if i != t.fields.len() - 1 {
											format!("{}, ", type_name(f, metadata))
										} else {
											format!("{}", type_name(f, metadata))
										}
									})
									.collect::<String>()
							)
						},
						TypeDef::Compact(t) => {
							format!("{:?}", type_name(t.type_param, metadata))
						},
						TypeDef::BitSequence(_) => "Not support BitSequence now".to_string(),
						_ => "Unexpected def type".to_string(),
					}
				}
			},
			None => "Unknown type".to_string(),
		}
	}

	match entry_type {
		StorageEntryType::Plain(t) => type_name(t.into(), metadata),
		StorageEntryType::Map { hashers: _, key_ty, value_ty } => format!(
			"{} -> {}",
			type_name(key_ty.into(), metadata),
			type_name(value_ty.into(), metadata)
		),
	}
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

pub fn print_usage<T: clap::Subcommand>(command_name: Str) {
	let mock = Command::new(command_name)
		.disable_help_flag(true)
		.disable_help_subcommand(true)
		.no_binary_name(true);
	let mut command = <T as clap::Subcommand>::augment_subcommands(mock);
	println!("{}", command.render_long_help());
}
