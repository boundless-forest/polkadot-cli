use core::any::TypeId;
use frame_metadata::v14::{RuntimeMetadataV14, StorageEntryType};
use scale_info::{form::PortableForm, interner::UntrackedSymbol, TypeDef};

pub fn print_storage_type(
	entry_type: &StorageEntryType<PortableForm>,
	metadata: &RuntimeMetadataV14,
) -> String {
	fn type_name(sym: &UntrackedSymbol<TypeId>, metadata: &RuntimeMetadataV14) -> String {
		match metadata.types.resolve(sym.id) {
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
							format!("[{}; {}]", type_name(&t.type_param, &metadata), t.len)
						},
						TypeDef::Sequence(t) => {
							format!("Vec<{}>", type_name(&t.type_param, &metadata))
						},
						TypeDef::Tuple(t) => {
							format!(
								"({})",
								t.fields
									.iter()
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
							format!("{:?}", type_name(&t.type_param, &metadata))
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
		StorageEntryType::Plain(t) => type_name(t, metadata),
		StorageEntryType::Map { hashers, key, value } =>
			format!("{} -> {}", type_name(key, metadata), type_name(value, metadata)),
	}
}
