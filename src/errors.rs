// crates.io
use rustyline::error::ReadlineError;
use serde::Serialize;
use thiserror::Error;
// this crate
use crate::rpc::RpcError;

/// Application error type.
#[derive(Debug, Error, Serialize)]
pub enum AppError {
	/// RPC Error
	#[error("rpc error happens, err: {0}")]
	Rpc(RpcError),
	/// Readline Error
	#[error("readline error happens, err: {0}")]
	Readline(String),
	/// Custom error
	#[error("custom error happens, err: {0}")]
	Custom(String),
}

impl From<ReadlineError> for AppError {
	fn from(err: ReadlineError) -> Self {
		AppError::Readline(err.to_string())
	}
}

impl From<RpcError> for AppError {
	fn from(err: RpcError) -> Self {
		AppError::Rpc(err)
	}
}
