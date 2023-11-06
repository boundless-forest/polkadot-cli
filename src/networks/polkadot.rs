// crates.io
use sp_runtime::{
	generic::{Block, Header},
	traits::{BlakeTwo256, Hash as HashT, IdentifyAccount, Verify},
	MultiSignature, OpaqueExtrinsic,
};
// this crate
use super::{ChainInfo, Network};

type Signature = MultiSignature;
type AccountPublic = <Signature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

/// Polkadot Chain information
pub struct PolkadotChain;

impl ChainInfo for PolkadotChain {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Polkadot;
	const WS_PORT: &'static str = "wss://rpc.polkadot.io:443";
}

/// Kusama Chain information
pub struct KusamaChain;

impl ChainInfo for KusamaChain {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Kusama;
	const WS_PORT: &'static str = "wss://kusama-rpc.polkadot.io:443";
}
