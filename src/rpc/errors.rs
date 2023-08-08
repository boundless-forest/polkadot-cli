// crates.io
use serde::Serialize;
use thiserror::Error;

/// RPC error type.
#[derive(Debug, Error, Serialize)]
pub enum RpcError {
	#[error("failed to create an RPC client")]
	WsHandshakeError,
	#[error("not able to retrieve the rpc responds, err: {0}")]
	JsonRpseeError(String),
	#[error("invalid params, please check your parameters")]
	InvalidParams,
	#[error("decode error happens")]
	DecodeError,
	#[error("failed to generate storage key")]
	GenerateStorageKeyFailed,
	#[error("no result found")]
	EmptyResult,
	#[error("failed to parse the data to json")]
	InvalidJsonObject,
}

impl From<jsonrpsee::core::Error> for RpcError {
	fn from(err: jsonrpsee::core::Error) -> Self {
		RpcError::JsonRpseeError(err.to_string())
	}
}
