// crates.io
use sp_runtime::{
	generic::{Block, Header},
	traits::{BlakeTwo256, Hash as HashT},
	OpaqueExtrinsic,
};
// this crate
use super::{ChainInfo, Network};

type Signature = fp_account::EthereumSignature;
type AccountPublic = <Signature as sp_runtime::traits::Verify>::Signer;
pub type AccountId = <AccountPublic as sp_runtime::traits::IdentifyAccount>::AccountId;

/// Pangolin Chain information
pub struct PangolinChain;

impl ChainInfo for PangolinChain {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Pangolin;
	const WS_PORT: &'static str = "wss://pangolin-rpc.darwinia.network:443";
}

/// Pangoro Chain information
pub struct PangoroChain;

impl ChainInfo for PangoroChain {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Pangoro;
	const WS_PORT: &'static str = "wss://pangoro-rpc.darwinia.network:443";
}

/// Crab Chain information
pub struct CrabChain;

impl ChainInfo for CrabChain {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Crab;
	const WS_PORT: &'static str = "wss://crab-rpc.darwinia.network:443";
}

/// Darwinia Chain information
pub struct DarwiniaChain;

impl ChainInfo for DarwiniaChain {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Darwinia;
	const WS_PORT: &'static str = "wss://rpc.darwinia.network:443";
}
