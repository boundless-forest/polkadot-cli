// crates.io
use frame_metadata::{
	v14::{StorageEntryType, StorageHasher},
	RuntimeMetadata,
};
use sp_core::Encode;
use sp_storage::StorageKey;

/// Get the storage key with the provided pallet information and runtime metadata.
pub fn single_map_storage_key<K: Encode>(
	runtime_metadata: &RuntimeMetadata,
	pallet_name: &str,
	storage_name: &str,
	key: K,
) -> Result<StorageKey, String> {
	let RuntimeMetadata::V14(metadata) = &runtime_metadata  else {
		return Err("Only support the runtime metadata V14 now.".to_string());
	};
	let Some(p) = metadata.pallets.iter().find(|p| p.name == pallet_name) else {
		return Err("Did not find the pallet.".to_string());
	};
	let Some(entry) = p.storage.clone().map(|s| s.entries).and_then(|entries| entries.into_iter().find(|e| e.name == storage_name)) else {
		return Err("Did not find the storage item.".to_string());
	};
	let StorageEntryType::Map { hashers, key: _, value : _} = entry.ty else {
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
