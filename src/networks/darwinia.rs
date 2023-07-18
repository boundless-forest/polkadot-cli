// crates.io
use sp_core::H256;
use sp_runtime::{
	generic::{Block, Header},
	traits::BlakeTwo256,
	OpaqueExtrinsic as UncheckedExtrinsic,
};
// this crate
use super::{ChainInfo, Network};

pub struct PangolinChain;

impl ChainInfo for PangolinChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Pangolin;
	const WS_PORT: &'static str = "wss://pangolin-rpc.darwinia.network:443";
}

pub struct PangoroChain;

impl ChainInfo for PangoroChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Pangoro;
	const WS_PORT: &'static str = "wss://pangoro-rpc.darwinia.network:443";
}

pub struct CrabChain;

impl ChainInfo for CrabChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Crab;
	const WS_PORT: &'static str = "wss://crab-rpc.darwinia.network:443";
}

pub struct DarwiniaChain;

impl ChainInfo for DarwiniaChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Darwinia;
	const WS_PORT: &'static str = "wss://rpc.darwinia.network:443";
}
