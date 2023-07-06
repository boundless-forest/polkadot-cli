mod node_template;

pub use node_template::NoteTemplate;
use serde::Serialize;
use sp_runtime::DeserializeOwned;

use std::marker::Sync;

/// The ChainInfo API
pub trait ChainInfo: Sync + Send {
	/// The hash type of the chain
	type Hash: Serialize + DeserializeOwned + Send;
	/// The block number type of the chain
	type BlockNumber: Serialize + DeserializeOwned + Send;
	/// The header type of the chain
	type Header: Serialize + DeserializeOwned;
	/// The block type of the chain
	type Block: Serialize + DeserializeOwned;
}
