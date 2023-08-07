use jsonrpsee::core::Error;

/// RPC error type.
#[derive(Debug)]
pub enum RpcError {
	InvalidUri,
	WsHandshakeError,
	JsonRpseeError(Error),
	InvalidCommandParams,
	DecodeError,
	StorageKeyFailed,
}
