use core::any::TypeId;
use scale_info::{interner::UntrackedSymbol, TypeDef};
use subxt_metadata::{Metadata, StorageEntryType};

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
