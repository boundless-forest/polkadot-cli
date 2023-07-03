use jsonrpsee::core::Error;
use rustyline::error::ReadlineError;


/// Application error type.
#[derive(Debug)]
pub enum AppError {
	/// RPC Error
	RpcError(RpcError),
	/// Readline Error
	Readline(ReadlineError),
}

/// RPC error type.
#[derive(Debug)]
pub enum RpcError {
	InvalidUri,
	WsHandshakeError,
	JsonRpseeError(Error),
}
