// crates.io
use sp_core::H256;
use sp_runtime::{
	generic::{Block, Header},
	traits::BlakeTwo256,
	OpaqueExtrinsic as UncheckedExtrinsic,
};
// this crate
use super::ChainInfo;

pub struct PangolinChain;

impl ChainInfo for PangolinChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const WS_END_POINT: &'static str = "wss://pangolin-rpc.darwinia.network:443";
}

pub struct PangoroChain;

impl ChainInfo for PangoroChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const WS_END_POINT: &'static str = "wss://pangoro-rpc.darwinia.network:443";
}

pub struct CrabChain;

impl ChainInfo for CrabChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const WS_END_POINT: &'static str = "wss://crab-rpc.darwinia.network:443";
}

pub struct DarwiniaChain;

impl ChainInfo for DarwiniaChain {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const WS_END_POINT: &'static str = "wss://rpc.darwinia.network:443";
}
