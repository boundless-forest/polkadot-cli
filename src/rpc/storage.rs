// crates.io
use sp_core::{twox_128, Encode};
use sp_storage::StorageKey;
use subxt_metadata::{Metadata, StorageEntryType, StorageHasher};

/// Get the storage key with the provided pallet information and runtime metadata.
pub fn single_map_storage_key<K: Encode>(
	runtime_metadata: &Metadata,
	pallet_name: &str,
	storage_name: &str,
	key: K,
) -> Result<StorageKey, String> {
	let Some(p) = runtime_metadata.pallets().find(|p| p.name() == pallet_name) else {
		return Err("Did not find the pallet.".to_string());
	};
	let Some(entry) = p.storage().map(|s| s.entries()).and_then(|entries| entries.iter().find(|e| e.name() == storage_name)) else {
		return Err("Did not find the storage item.".to_string());
	};
	let StorageEntryType::Map { hashers, key_ty: _, value_ty: _} = entry.entry_type() else {
		return Err("Only support single map entry in this function".to_string());
	};

	let mut storage_key = sp_core::twox_128(pallet_name.as_bytes()).to_vec();
	storage_key.extend(&sp_core::twox_128(storage_name.as_bytes()));

	let hasher = hashers.get(0).expect("Failed to get hasher");
	storage_key.extend(key_hash(&key, hasher));

	Ok(StorageKey(storage_key))
}

/// generates the key's hash depending on the StorageHasher selected
fn key_hash<K: Encode>(key: &K, hasher: &StorageHasher) -> Vec<u8> {
	let encoded_key = key.encode();
	match hasher {
		StorageHasher::Identity => encoded_key.to_vec(),
		StorageHasher::Blake2_128 => sp_core::blake2_128(&encoded_key).to_vec(),
		StorageHasher::Blake2_128Concat => {
			// copied from substrate Blake2_128Concat::hash since StorageHasher is not public
			let x: &[u8] = encoded_key.as_slice();
			sp_core::blake2_128(x).iter().chain(x.iter()).cloned().collect::<Vec<_>>()
		},
		StorageHasher::Blake2_256 => sp_core::blake2_256(&encoded_key).to_vec(),
		StorageHasher::Twox128 => sp_core::twox_128(&encoded_key).to_vec(),
		StorageHasher::Twox256 => sp_core::twox_256(&encoded_key).to_vec(),
		StorageHasher::Twox64Concat =>
			sp_core::twox_64(&encoded_key).iter().chain(&encoded_key).cloned().collect(),
	}
}

pub fn events_storage_key() -> StorageKey {
	let mut key = twox_128("System".as_bytes()).to_vec();
	key.extend(twox_128("Events".as_bytes()));
	StorageKey(key)
}
