// std
use std::io::Error as IOError;
// crates.io
use rustyline::error::ReadlineError;
use serde::Serialize;
use thiserror::Error;
// this crate
use crate::rpc::RpcError;

/// Application error type.
#[derive(Debug, Error)]
pub enum AppError {
	/// RPC Error
	#[error("rpc error happens, err: {0}")]
	Rpc(RpcError),
	/// Readline Error
	#[error("readline error happens, err: {0}")]
	Readline(String),
	#[error("io error happens, err: {0}")]
	IO(IOError),
	/// Custom error
	#[error("custom error happens, err: {0}")]
	Custom(String),
}

impl Serialize for AppError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(self.to_string().as_str())
	}
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

impl From<IOError> for AppError {
	fn from(err: IOError) -> Self {
		AppError::IO(err)
	}
}
