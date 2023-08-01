// crates.io
use sp_core::H256;
use sp_runtime::{
	generic::{Block, Header},
	traits::BlakeTwo256,
	OpaqueExtrinsic as UncheckedExtrinsic,
};
// this crate
use super::{ChainInfo, Network};

/// Pangolin Chain information
pub struct PangolinChain;

impl ChainInfo for PangolinChain {
	type Balance = u128;
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Pangolin;
	const WS_PORT: &'static str = "wss://pangolin-rpc.darwinia.network:443";
}

/// Pangoro Chain information
pub struct PangoroChain;

impl ChainInfo for PangoroChain {
	type Balance = u128;
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Pangoro;
	const WS_PORT: &'static str = "wss://pangoro-rpc.darwinia.network:443";
}

/// Crab Chain information
pub struct CrabChain;

impl ChainInfo for CrabChain {
	type Balance = u128;
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Crab;
	const WS_PORT: &'static str = "wss://crab-rpc.darwinia.network:443";
}

/// Darwinia Chain information
pub struct DarwiniaChain;

impl ChainInfo for DarwiniaChain {
	type Balance = u128;
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Darwinia;
	const WS_PORT: &'static str = "wss://rpc.darwinia.network:443";
}
