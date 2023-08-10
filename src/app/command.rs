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
	/// Chain interfaces
	#[command(subcommand)]
	Chain(ChainCommand),
	/// Chain state interfaces
	#[command(subcommand)]
	State(StateCommand),
	/// Account interfaces
	#[command(subcommand)]
	AccountInfo(AccountInfoCommand),
	/// Runtime interfaces
	#[command(subcommand)]
	Runtime(RuntimeCommand), // TODO: ADD HELP COMMAND
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "rpc")]
pub enum RpcCommand {
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
	/// Get the finalized head number
	GetFinalizedNumber,
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
#[command(name = "account-info")]
pub enum AccountInfoCommand {
	Balances {
		#[arg(name = "account-id", long)]
		account_id: String,
		#[arg(name = "at-block", long)]
		at_block: Option<String>,
	},
	Nonce {
		#[arg(name = "account-id", long)]
		account_id: String,
		#[arg(name = "at-block", long)]
		at_block: Option<String>,
	},
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "pallets")]
pub enum RuntimeCommand {
	ListPallets,
	ListPalletStorages {
		#[arg(name = "pallet-name", long)]
		pallet_name: String,
	},
	ListPalletConstants {
		#[arg(name = "pallet-name", long)]
		pallet_name: String,
	},
}
