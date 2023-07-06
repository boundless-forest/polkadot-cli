// crates.io
use async_trait::async_trait;
// this crate
use super::{
	rpc::RpcResult,
	types::{ChainType, Health, Properties},
};

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
