// crates.io
use sp_runtime::{
	generic::{Block, Header},
	traits::{BlakeTwo256, Hash as HashT, IdentifyAccount, Verify},
	MultiSignature, OpaqueExtrinsic,
};
// this crate
use super::{ChainInfo, Network};

pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

pub struct NoteTemplate;

impl ChainInfo for NoteTemplate {
	type AccountId = AccountId;
	type Balance = u128;
	type Block = Block<Self::Header, OpaqueExtrinsic>;
	type BlockNumber = u32;
	type Hash = <Self::Hashing as HashT>::Output;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, Self::Hashing>;
	type Nonce = u32;

	const NET_WORK: Network = Network::Local;
	const WS_PORT: &'static str = "ws://127.0.0.1:9944";
}
