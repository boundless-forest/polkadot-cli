// crates.io
use sp_core::H256;
use sp_runtime::{
	generic::{Block, Header},
	traits::BlakeTwo256,
	OpaqueExtrinsic as UncheckedExtrinsic,
};
// this crate
use super::ChainInfo;

pub struct NoteTemplate;

impl ChainInfo for NoteTemplate {
	type Block = Block<Self::Header, UncheckedExtrinsic>;
	type BlockNumber = u32;
	type Hash = H256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;
}