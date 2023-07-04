// crates.io
use jsonrpsee::core::Error;
use rustyline::error::ReadlineError;

/// Application error type.
#[derive(Debug)]
pub enum AppError {
	/// RPC Error
	Rpc(RpcError),
	/// Readline Error
	Readline(ReadlineError),
	/// Handler Error
	Handler(HandlerError),
	/// Custom error
	Custom(String),
}

impl From<ReadlineError> for AppError {
	fn from(err: ReadlineError) -> Self {
		AppError::Readline(err)
	}
}

/// RPC error type.
#[derive(Debug)]
pub enum RpcError {
	InvalidUri,
	WsHandshakeError,
	JsonRpseeError(Error),
	InvalidRpcCommand,
}

impl From<RpcError> for AppError {
	fn from(err: RpcError) -> Self {
		AppError::Rpc(err)
	}
}

/// Handler error type.
#[derive(Debug)]
pub enum HandlerError {
	UnknownAppCommand,
}

impl From<HandlerError> for AppError {
	fn from(err: HandlerError) -> Self {
		AppError::Handler(err)
	}
}
