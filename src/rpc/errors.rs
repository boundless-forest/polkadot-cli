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
	/// Error working with metadata.
	#[error("Metadata error: {0}")]
	Metadata(#[from] MetadataError),
}

/// Something went wrong trying to access details in the metadata.
#[derive(Clone, Debug, PartialEq, thiserror::Error, Serialize)]
#[non_exhaustive]
pub enum MetadataError {
	/// The DispatchError type isn't available in the metadata
	#[error("The DispatchError type isn't available")]
	DispatchErrorNotFound,
	/// Type not found in metadata.
	#[error("Type with ID {0} not found")]
	TypeNotFound(u32),
	/// Pallet not found (index).
	#[error("Pallet with index {0} not found")]
	PalletIndexNotFound(u8),
	/// Pallet not found (name).
	#[error("Pallet with name {0} not found")]
	PalletNameNotFound(String),
	/// Variant not found.
	#[error("Variant with index {0} not found")]
	VariantIndexNotFound(u8),
	/// Constant not found.
	#[error("Constant with name {0} not found")]
	ConstantNameNotFound(String),
	/// Call not found.
	#[error("Call with name {0} not found")]
	CallNameNotFound(String),
	/// Runtime trait not found.
	#[error("Runtime trait with name {0} not found")]
	RuntimeTraitNotFound(String),
	/// Runtime method not found.
	#[error("Runtime method with name {0} not found")]
	RuntimeMethodNotFound(String),
	/// Call type not found in metadata.
	#[error("Call type not found in pallet with index {0}")]
	CallTypeNotFoundInPallet(u8),
	/// Event type not found in metadata.
	#[error("Event type not found in pallet with index {0}")]
	EventTypeNotFoundInPallet(u8),
	/// Storage details not found in metadata.
	#[error("Storage details not found in pallet with name {0}")]
	StorageNotFoundInPallet(String),
	/// Storage entry not found.
	#[error("Storage entry {0} not found")]
	StorageEntryNotFound(String),
	/// The generated interface used is not compatible with the node.
	#[error("The generated code is not compatible with the node")]
	IncompatibleCodegen,
	/// Custom value not found.
	#[error("Custom value with name {0} not found")]
	CustomValueNameNotFound(String),
}

impl From<jsonrpsee::core::Error> for RpcError {
	fn from(err: jsonrpsee::core::Error) -> Self {
		RpcError::JsonRpseeError(err.to_string())
	}
}
