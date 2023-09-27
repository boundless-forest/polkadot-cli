// crates.io
use async_trait::async_trait;
use jsonrpsee::{
	core::{client::Subscription, traits::ToRpcParams},
	rpc_params,
};
use serde::de::DeserializeOwned;
use sp_core::Decode;
use sp_runtime::generic::SignedBlock;
use sp_storage::StorageKey;
use sp_version::RuntimeVersion;
use subxt_metadata::Metadata;
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

/// The State API provides access to chain state and storage.
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
	async fn runtime_metadata(&self) -> RpcResult<Metadata>;

	/// Retrieves the storage for a key
	async fn get_storage<R: Decode>(
		&self,
		storage_key: StorageKey,
		at_block: Option<HashForChain<Self::ChainInfo>>,
	) -> RpcResult<Option<R>>;
}

#[async_trait]
pub trait SubscribeApi {
	/// The chain info type
	type ChainInfo: ChainInfo;

	async fn subscribe<Params, Notif>(
		&self,
		subscribe_method: &str,
		params: Params,
		unsubscribe_method: &str,
	) -> RpcResult<Subscription<Notif>>
	where
		Params: ToRpcParams + Send,
		Notif: DeserializeOwned;

	async fn subscribe_finalized_heads(
		&self,
	) -> RpcResult<Subscription<HeaderForChain<Self::ChainInfo>>> {
		self.subscribe(
			"chain_subscribeFinalizedHeads",
			rpc_params![],
			"chain_unsubscribeFinalizedHeads",
		)
		.await
		.map_err(|e| e.into())
	}
}
