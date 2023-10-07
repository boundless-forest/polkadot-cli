// std
use std::{marker::PhantomData, sync::Arc};
// crates.io
use async_trait::async_trait;
use frame_metadata::RuntimeMetadataPrefixed;
use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::client::{Client, ClientBuilder, ClientT},
	rpc_params,
};
use sp_core::{Bytes, Decode};
use sp_runtime::generic::SignedBlock;
use sp_storage::{StorageData, StorageKey};
use sp_version::RuntimeVersion;
use subxt_metadata::Metadata;
// this crate
use super::{
	api::{
		BlockForChain, BlockNumberForChain, ChainApi, HashForChain, HeaderForChain, StateApi,
		SystemApi,
	},
	errors::RpcError,
	types::{this_crate_types::SystemPaneInfo, ChainType, Health, Properties},
};
use crate::networks::ChainInfo;

/// RPC result type.
pub type RpcResult<T> = Result<T, RpcError>;

/// RPC client.
pub struct RpcClient<CI> {
	pub client: Arc<Client>,
	_chain_info: PhantomData<CI>,
}

impl<CI: ChainInfo> RpcClient<CI> {
	/// Create a new RPC client with given URL.
	pub async fn new() -> RpcResult<Self> {
		let (tx, rx) = WsTransportClientBuilder::default()
			.build(Uri::from_static(<CI as ChainInfo>::WS_PORT))
			.await
			.map_err(|_| RpcError::WsHandshakeError)?;
		let client = ClientBuilder::default().build_with_tokio(tx, rx);
		Ok(Self { client: Arc::new(client), _chain_info: PhantomData })
	}

	pub async fn system_pane_info(&self) -> RpcResult<SystemPaneInfo> {
		let system_name = self.system_name().await?;
		let system_version = self.system_version().await?;
		let chain_type = self.chain_type().await?.to_string();
		let chain_name = self.chain().await?;
		let properties = self.system_properties().await?;
		let token_decimal = properties
			.get("tokenDecimals")
			.expect("Failed to get token decimals")
			.to_string();
		let token_symbol = properties
			.get("tokenSymbol")
			.expect("Failed to get token decimals")
			.to_string()
			.trim_matches('\"')
			.to_string();

		let hash = self.get_finalized_head().await?.expect("Failed to get finalized head");
		let runtime_version = self.runtime_version(hash).await?;
		Ok(SystemPaneInfo {
			system_name,
			system_version,
			chain_type,
			chain_name,
			token_symbol,
			token_decimal,
			runtime_version,
		})
	}
}

#[async_trait]
impl<CI: ChainInfo> SystemApi for RpcClient<CI> {
	/// Get the node name.
	async fn system_name(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_name", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the node properties.
	async fn system_properties(&self) -> RpcResult<Properties> {
		let res = self
			.client
			.request("system_properties", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the node version.
	async fn system_version(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_version", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the chain name
	async fn chain(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_chain", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the chain type
	async fn chain_type(&self) -> RpcResult<ChainType> {
		let res = self
			.client
			.request("system_chainType", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the chain health status
	async fn health(&self) -> RpcResult<Health> {
		let res = self
			.client
			.request("system_health", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the chain sync status
	async fn sync_state(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_syncState", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}
}

#[async_trait]
impl<CI: ChainInfo> ChainApi for RpcClient<CI> {
	type ChainInfo = CI;

	/// Get the chain block
	async fn get_block(
		&self,
		hash: HashForChain<Self::ChainInfo>,
	) -> RpcResult<SignedBlock<BlockForChain<Self::ChainInfo>>> {
		let res = self
			.client
			.request("chain_getBlock", rpc_params![hash])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the block hash for a specific block
	async fn get_block_hash(
		&self,
		number: BlockNumberForChain<Self::ChainInfo>,
	) -> RpcResult<Option<HashForChain<Self::ChainInfo>>> {
		let res = self
			.client
			.request("chain_getBlockHash", rpc_params![number])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the hash of the last finalized block in the canon chain
	async fn get_finalized_head(&self) -> RpcResult<Option<HashForChain<Self::ChainInfo>>> {
		let res = self
			.client
			.request("chain_getFinalizedHead", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Retrieves the header for a specific block
	async fn get_header(
		&self,
		hash: HashForChain<Self::ChainInfo>,
	) -> RpcResult<HeaderForChain<Self::ChainInfo>> {
		let res = self
			.client
			.request("chain_getHeader", rpc_params![hash])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}
}

#[async_trait]
impl<CI: ChainInfo> StateApi for RpcClient<CI> {
	type ChainInfo = CI;

	/// Get the runtime version
	async fn runtime_version(
		&self,
		hash: HashForChain<Self::ChainInfo>,
	) -> RpcResult<RuntimeVersion> {
		let res = self
			.client
			.request("state_getRuntimeVersion", rpc_params![hash])
			.await
			.map_err(RpcError::from)?;
		Ok(res)
	}

	/// Get the runtime metadata
	async fn runtime_metadata(&self) -> RpcResult<Metadata> {
		let metadata_bytes: Bytes = self
			.client
			.request("state_getMetadata", rpc_params![])
			.await
			.map_err(RpcError::from)?;
		let metadata_prefix = RuntimeMetadataPrefixed::decode(&mut metadata_bytes.0.as_slice())
			.map_err(|_| RpcError::DecodeError)?;
		let metadata = metadata_prefix.try_into().map_err(|_| RpcError::DecodeError)?;

		Ok(metadata)
	}

	/// Retrieves the storage for a key
	async fn get_storage<R: Decode>(
		&self,
		storage_key: StorageKey,
		at_block: Option<HashForChain<Self::ChainInfo>>,
	) -> RpcResult<Option<R>> {
		let storage_data: Option<StorageData> = self
			.client
			.request("state_getStorage", rpc_params![storage_key, at_block])
			.await
			.map_err(RpcError::from)?;

		if let Some(data) = storage_data {
			return Ok(Some(R::decode(&mut data.0.as_slice()).map_err(|_| RpcError::DecodeError)?));
		}
		Ok(None)
	}
}
