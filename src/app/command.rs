// this crate
use crate::Network;
// crates.io
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(no_binary_name = true)]
pub enum AppCommand {
	/// Switch network, default is local
	#[command(subcommand)]
	SwitchNetwork(Network),
	/// RPC interfaces
	#[command(subcommand)]
	Rpc(RpcCommand),
	#[command(subcommand)]
	Chain(ChainCommand),
	#[command(subcommand)]
	State(StateCommand),
	#[command(subcommand)]
	Account(AccountCommand),
	// TODO: ADD HELP COMMAND
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "rpc")]
pub enum RpcCommand {
	/// Get RPC methods
	// RpcMethods,
	/// Get the node name
	SysName,
	/// Get System Properties
	SysProperties,
	/// Get System Version
	SysVersion,
	/// Get the chain
	Chain,
	/// Get the chain type
	ChainType,
	/// Get the health status of the node
	Health,
	/// Get the state of the syncing of the node
	SyncState,
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "chain")]
#[allow(clippy::enum_variant_names)]
pub enum ChainCommand {
	/// Get the chain block
	GetBlock {
		#[arg(long)]
		hash: String,
	},
	/// Get the block hash
	GetBlockHash {
		#[arg(long)]
		number: u32,
	},
	/// Get the finalized head hash
	GetFinalizedHead,
	/// Get the header for a specific block
	GetHeader {
		#[arg(long)]
		hash: String,
	},
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "state")]
pub enum StateCommand {
	RuntimeVersion {
		#[arg(long)]
		hash: Option<String>,
	},
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "account")]
pub enum AccountCommand {
	Balances {
		#[arg(long)]
		account: String,
	},
}
