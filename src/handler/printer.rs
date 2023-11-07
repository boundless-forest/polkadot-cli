// crates.io
use clap::{builder::Str, Command};
use colored::Colorize;
use core::any::TypeId;
use scale_info::{interner::UntrackedSymbol, TypeDef};
use serde::Serialize;
use subxt_metadata::{Metadata, StorageEntryType};
// this crate
use crate::rpc::{RpcError, RpcResult};

const VEC_WRAPPER: [&str; 3] = ["BoundedVec", "WeakBoundedVec", "Option"];

/// Fetch the storage type string.
pub fn print_storage_type(entry_type: StorageEntryType, metadata: &Metadata) -> String {
	fn ty_name(sym: UntrackedSymbol<TypeId>, metadata: &Metadata) -> String {
		match metadata.types().resolve(sym.id) {
			Some(ty) => {
				if let Some(i) = ty.path.ident() {
					log::debug!(target: "cli", "ident: {:?}", i);

					if VEC_WRAPPER.contains(&i.as_str()) {
						let inner_ty = ty.type_params[0].ty.expect("This type missed generic type T");
						format!("{}<{}>", i, ty_name(inner_ty, metadata))
					} else {
						format!("{}", i)
					}
				} else {
					// Need to check the real type
					match ty.type_def.clone() {
						TypeDef::Primitive(t) => {
							log::debug!(target: "cli", "Primitive: {:?}", t);
							format!("{:?}", t)
						},
						TypeDef::Array(t) => {
							format!("[{}; {}]", ty_name(t.type_param, metadata), t.len)
						},
						TypeDef::Sequence(t) => {
							format!("Vec<{}>", ty_name(t.type_param, metadata))
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
											format!("{}, ", ty_name(f, metadata))
										} else {
											format!("{}", ty_name(f, metadata))
										}
									})
									.collect::<String>()
							)
						},
						TypeDef::Compact(t) => {
							format!("{:?}", ty_name(t.type_param, metadata))
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
		StorageEntryType::Plain(t) => ty_name(t.into(), metadata),
		StorageEntryType::Map { hashers: _, key_ty, value_ty } =>
			format!("{} -> {}", ty_name(key_ty.into(), metadata), ty_name(value_ty.into(), metadata)),
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
