// std
use std::{marker::PhantomData, sync::Arc};
// crates.io
use async_trait::async_trait;
use colored::Colorize;
use frame_metadata::OpaqueMetadata;
use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::client::{Client, ClientBuilder, ClientT},
	rpc_params,
};
use serde::Serialize;
use sp_runtime::generic::SignedBlock;
use sp_version::RuntimeVersion;
// this crate
use super::{
	api::{
		BlockForChain, BlockNumberForChain, ChainApi, HashForChain, HeaderForChain, StateApi,
		SystemApi,
	},
	types::{ChainType, Health, Properties},
};
use crate::{errors::RpcError, networks::ChainInfo};

/// RPC result type.
pub type RpcResult<T> = Result<T, RpcError>;

/// RPC client.
#[derive(Clone)]
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
}

#[async_trait]
impl<CI: ChainInfo> SystemApi for RpcClient<CI> {
	/// Get the node RPC methods.
	async fn rpc_methods(&self) -> RpcResult<Vec<String>> {
		let res = self
			.client
			.request("rpc_methods", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the node name.
	async fn system_name(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_name", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the node properties.
	async fn system_properties(&self) -> RpcResult<Properties> {
		let res = self
			.client
			.request("system_properties", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the node version.
	async fn system_version(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_version", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the chain name
	async fn chain(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_chain", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the chain type
	async fn chain_type(&self) -> RpcResult<ChainType> {
		let res = self
			.client
			.request("system_chainType", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the chain health status
	async fn health(&self) -> RpcResult<Health> {
		let res = self
			.client
			.request("system_health", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the chain sync status
	async fn sync_state(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_syncState", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
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
			.map_err(RpcError::JsonRpseeError)?;
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
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the hash of the last finalized block in the canon chain
	async fn get_finalized_head(&self) -> RpcResult<Option<HashForChain<Self::ChainInfo>>> {
		let res = self
			.client
			.request("chain_getFinalizedHead", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
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
			.map_err(RpcError::JsonRpseeError)?;
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
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	/// Get the runtime metadata
	async fn runtime_metadata(&self) -> RpcResult<OpaqueMetadata> {
		// TODO: change the return type to concrete type
		let res = self
			.client
			.request("state_getMetadata", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}
}

/// Print the result in JSON format.
pub fn print_format_json<T: Serialize>(data: T) {
	if let Ok(data) = serde_json::to_string_pretty(&data) {
		println!("{}", data.italic().bright_magenta());
	} else {
		println!("{}", "Failed to format JSON".italic().bright_magenta());
	}
}
