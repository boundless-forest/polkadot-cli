use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::{
		client::{Client, ClientBuilder, ClientT},
		Error,
	},
	rpc_params,
};

/// RPC result type.
pub type RpcResult<T> = Result<T, RpcError>;

/// RPC client.
#[derive(Clone)]
pub struct RpcClient {
	pub client: Arc<Client>,
}

impl RpcClient {
	/// Create a new RPC client with given URL.
	pub async fn new(url: &str) -> RpcResult<Self> {
		let uri: Uri = format!("ws://{}", url).parse().map_err(|_| RpcError::InvalidUri)?;
		let (tx, rx) = WsTransportClientBuilder::default()
			.build(uri)
			.await
			.map_err(|_| RpcError::WsHandshakeError)?;
		let client = ClientBuilder::default().build_with_tokio(tx, rx);
		Ok(Self { client: Arc::new(client) })
	}

	/// Create a new RPC client with default URL.
	pub async fn with_default_url() -> RpcResult<Self> {
		Self::new("ws://127.0.0.1:9944").await
	}
}

#[async_trait]
pub trait Api {
	/// Get the node RPC methods.
	async fn rpc_methods(&self) -> RpcResult<Vec<String>>;

	/// Get the node name.
	async fn system_name(&self) -> RpcResult<String>;

	/// Get the node properties.
	async fn system_properties(&self) -> RpcResult<String>;

	/// Get the node version.
	async fn system_version(&self) -> RpcResult<String>;
}

#[async_trait]
impl Api for RpcClient {
	async fn rpc_methods(&self) -> RpcResult<Vec<String>> {
		let res = self
			.client
			.request("system_name", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	async fn system_name(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_name", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	async fn system_properties(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_name", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}

	async fn system_version(&self) -> RpcResult<String> {
		let res = self
			.client
			.request("system_version", rpc_params![])
			.await
			.map_err(RpcError::JsonRpseeError)?;
		Ok(res)
	}
}
