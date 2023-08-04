// crates.io
use serde::{Deserialize, Serialize};

/// Arbitrary properties defined in chain spec as a JSON object
// https://github.com/paritytech/substrate/blob/c172d0f683fab3792b90d876fd6ca27056af9fe9/client/chain-spec/src/lib.rs#L215-L216
pub type Properties = serde_json::map::Map<String, serde_json::Value>;

/// The type of a chain.
///
/// This can be used by tools to determine the type of a chain for displaying
/// additional information or enabling additional features.
// https://github.com/paritytech/substrate/blob/c172d0f683fab3792b90d876fd6ca27056af9fe9/client/chain-spec/src/lib.rs#L193-L207
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ChainType {
	/// A development chain that runs mainly on one node.
	Development,
	/// A local chain that runs locally on multiple nodes for testing purposes.
	Local,
	/// A live chain.
	Live,
	/// Some custom chain type.
	Custom(String),
}

/// Health struct returned by the RPC
// https://github.com/paritytech/substrate/blob/c172d0f683fab3792b90d876fd6ca27056af9fe9/client/rpc-api/src/system/helpers.rs#L40-L58
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
	/// Number of connected peers
	pub peers: usize,
	/// Is the node syncing
	pub is_syncing: bool,
	/// Should this node have any peers
	///
	/// Might be false for local chains or when running without discovery.
	pub should_have_peers: bool,
}

pub mod this_crate_types {
	use super::*;

	/// Account balances output format
	#[derive(Serialize, Deserialize)]
	pub struct AccountBalances<Balance> {
		pub free: Balance,
		pub reserved: Balance,
		pub frozen: Balance,
	}
}
