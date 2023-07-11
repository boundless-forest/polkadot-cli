mod darwinia;
mod node_template;

use anyhow::Chain;
pub use darwinia::{CrabChain, DarwiniaChain, PangolinChain, PangoroChain};
pub use node_template::NoteTemplate;

// std
use std::{marker::Sync, str::FromStr};
// this crate
use crate::app::Network;
// crates.io
use serde::Serialize;
use sp_runtime::DeserializeOwned;

/// The ChainInfo API
pub trait ChainInfo: Sync + Send {
	/// The ws endpoint of the chain
	const WS_END_POINT: &'static str;
	/// The network name of this chain
	// TODO: FIX ME
	const NET_WORK: Network;

	/// The hash type of the chain
	type Hash: Serialize + DeserializeOwned + Send + FromStr;
	/// The block number type of the chain
	type BlockNumber: Serialize + DeserializeOwned + Send + From<u32>;
	/// The header type of the chain
	type Header: Serialize + DeserializeOwned;
	/// The block type of the chain
	type Block: Serialize + DeserializeOwned;
}

pub enum Chains {
	Local(NoteTemplate),
	Pangolin(PangolinChain),
	Pangoro(PangoroChain),
	Crab(CrabChain),
	Darwinia(DarwiniaChain),
}

pub fn chain_info(network: Network) -> Chains {
	match network {
		Network::Crab => Chains::Crab(CrabChain {}),
		Network::Darwinia => Chains::Darwinia(DarwiniaChain {}),
		Network::Pangolin => Chains::Pangolin(PangolinChain {}),
		Network::Pangoro => Chains::Pangoro(PangoroChain {}),
		Network::Local => Chains::Local(NoteTemplate {}),
		_ => unreachable!(),
	}
}
