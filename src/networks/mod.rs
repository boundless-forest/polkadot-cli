mod darwinia;
mod node_template;
mod polkadot;

pub use darwinia::{CrabChain, DarwiniaChain, PangolinChain, PangoroChain};
pub use node_template::NoteTemplate;
pub use polkadot::{KusamaChain, PolkadotChain};

// std
use std::{fmt::Debug, marker::Sync, str::FromStr};
// crates.io
use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use sp_core::{Decode, Encode, H256};
use sp_runtime::{
	traits::{Block as BlockT, Hash as HashT, Header as HeaderT},
	DeserializeOwned,
};

/// The ChainInfo API
pub trait ChainInfo: Sync + Send {
	/// The ws endpoint of the chain
	const WS_PORT: &'static str;
	/// The network name of this chain
	const NET_WORK: Network;

	/// The account id type of the chain
	type AccountId: Serialize + DeserializeOwned + Encode + FromStr + AsRef<[u8]>;
	/// The balance type of the chain
	type Balance: Serialize + DeserializeOwned + Decode + Debug;
	/// The block type of the chain
	type Block: Serialize + DeserializeOwned + BlockT;
	/// The block number type of the chain
	type BlockNumber: Serialize + DeserializeOwned + Send + From<u32>;
	/// The hash type of the chain
	type Hash: Serialize
		+ DeserializeOwned
		+ Send
		+ FromStr
		+ From<H256>
		+ From<<Self::Header as HeaderT>::Hash>;
	type Hashing: HashT;
	/// The header type of the chain
	type Header: Serialize + DeserializeOwned + HeaderT;
	///  The nonce type of the chain
	type Nonce: Serialize + DeserializeOwned + Decode + Debug;
}

#[derive(Subcommand, Clone, Debug, Serialize, Deserialize, Default, ValueEnum)]
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
