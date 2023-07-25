// crates.io
use sp_core::H256;
use sp_runtime::{
	generic::{Block, Header},
	traits::BlakeTwo256,
	OpaqueExtrinsic as UncheckedExtrinsic,
};
// this crate
use super::{ChainInfo, Network};

/// Polkadot Chain information
pub struct PolkadotChain;

impl ChainInfo for PolkadotChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Polkadot;
	const WS_PORT: &'static str = "wss://rpc.polkadot.io:443";
}

/// Kusama Chain information
pub struct KusamaChain;

impl ChainInfo for KusamaChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Kusama;
	const WS_PORT: &'static str = "wss://kusama-rpc.polkadot.io:443";
}
