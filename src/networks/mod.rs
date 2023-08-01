mod darwinia;
mod node_template;
mod polkadot;

pub use darwinia::{CrabChain, DarwiniaChain, PangolinChain, PangoroChain};
pub use node_template::NoteTemplate;
pub use polkadot::{KusamaChain, PolkadotChain};

// std
use std::{marker::Sync, str::FromStr};
// crates.io
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use sp_runtime::DeserializeOwned;

/// The ChainInfo API
pub trait ChainInfo: Sync + Send {
	/// The ws endpoint of the chain
	const WS_PORT: &'static str;
	/// The network name of this chain
	const NET_WORK: Network;

	/// The balance type of the chain
	type Balance: Serialize + DeserializeOwned;
	/// The block type of the chain
	type Block: Serialize + DeserializeOwned;
	/// The block number type of the chain
	type BlockNumber: Serialize + DeserializeOwned + Send + From<u32>;
	/// The hash type of the chain
	type Hash: Serialize + DeserializeOwned + Send + FromStr;
	/// The header type of the chain
	type Header: Serialize + DeserializeOwned;
}

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize, Default)]
pub enum Network {
	Local,
	// parity
	Polkadot,
	Kusama,
	// Darwinia
	Pangolin,
	Pangoro,
	#[default]
	Darwinia,
	Crab,
}
