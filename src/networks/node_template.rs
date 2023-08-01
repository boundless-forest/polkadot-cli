// crates.io
use sp_core::H256;
use sp_runtime::{
	generic::{Block, Header},
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, OpaqueExtrinsic as UncheckedExtrinsic,
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
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;

	const NET_WORK: Network = Network::Local;
	const WS_PORT: &'static str = "ws://127.0.0.1:9944";
}
