// crates.io
use async_trait::async_trait;
use frame_metadata::OpaqueMetadata;
use sp_runtime::generic::SignedBlock;
use sp_version::RuntimeVersion;
// this crate
use super::{
	client::RpcResult,
	types::{ChainType, Health, Properties},
};
use crate::networks::ChainInfo;

/// Hash type fot the chain
pub type HashForChain<T> = <T as ChainInfo>::Hash;
/// Block number type fot the chain
pub type BlockNumberForChain<T> = <T as ChainInfo>::BlockNumber;
/// Header type fot the chain
pub type HeaderForChain<T> = <T as ChainInfo>::Header;
/// Block type fot the chain
pub type BlockForChain<T> = <T as ChainInfo>::Block;

/// The System API provides access to common system functions.
#[async_trait]
pub trait SystemApi {
	/// Get the node RPC methods.
	async fn rpc_methods(&self) -> RpcResult<Vec<String>>;
	/// Get the node name.
	async fn system_name(&self) -> RpcResult<String>;
	/// Get the node properties.
	async fn system_properties(&self) -> RpcResult<Properties>;
	/// Get the node version.
	async fn system_version(&self) -> RpcResult<String>;
	/// Get the chain name
	async fn chain(&self) -> RpcResult<String>;
	/// Get the chain type
	async fn chain_type(&self) -> RpcResult<ChainType>;
	/// Get the chain health status
	async fn health(&self) -> RpcResult<Health>;
	/// Get the chain sync status
	async fn sync_state(&self) -> RpcResult<String>;
}

/// The Chain API provides access to common chain functions.
#[async_trait]
pub trait ChainApi {
	/// The chain info type
	type ChainInfo: ChainInfo;

	/// Get the chain block
	async fn get_block(
		&self,
		hash: HashForChain<Self::ChainInfo>,
	) -> RpcResult<SignedBlock<<Self::ChainInfo as ChainInfo>::Block>>;

	/// Get the block hash for a specific block
	async fn get_block_hash(
		&self,
		number: BlockNumberForChain<Self::ChainInfo>,
	) -> RpcResult<Option<<Self::ChainInfo as ChainInfo>::Hash>>;

	/// Get the hash of the last finalized block in the canon chain
	async fn get_finalized_head(&self) -> RpcResult<Option<HashForChain<Self::ChainInfo>>>;

	/// Retrieves the header for a specific block
	async fn get_header(
		&self,
		hash: HashForChain<Self::ChainInfo>,
	) -> RpcResult<HeaderForChain<Self::ChainInfo>>;
}

#[async_trait]
pub trait StateApi {
	/// The chain info type
	type ChainInfo: ChainInfo;

	/// Get the runtime version	
	async fn runtime_version(
		&self,
		hash: HashForChain<Self::ChainInfo>,
	) -> RpcResult<RuntimeVersion>;

	/// Get the runtime metadata
	async fn runtime_metadata(&self) -> RpcResult<OpaqueMetadata>;
}
