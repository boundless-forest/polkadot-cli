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
}

/// RPC error type.
#[derive(Debug)]
pub enum RpcError {
	InvalidUri,
	WsHandshakeError,
	JsonRpseeError(Error),
}

/// Handler error type.
#[derive(Debug)]
pub enum HandlerError {
	InvalidAppCommand,
}
