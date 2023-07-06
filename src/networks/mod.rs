mod node_template;

pub use node_template::NoteTemplate;

// std
use std::{marker::Sync, str::FromStr};
// crates.io
use serde::Serialize;
use sp_runtime::DeserializeOwned;

/// The ChainInfo API
pub trait ChainInfo: Sync + Send {
	/// The hash type of the chain
	type Hash: Serialize + DeserializeOwned + Send + FromStr;
	/// The block number type of the chain
	type BlockNumber: Serialize + DeserializeOwned + Send + From<u32>;
	/// The header type of the chain
	type Header: Serialize + DeserializeOwned;
	/// The block type of the chain
	type Block: Serialize + DeserializeOwned;
}
