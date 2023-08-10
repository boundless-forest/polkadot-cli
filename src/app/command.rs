// this crate
use crate::Network;
// crates.io
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
#[clap(no_binary_name = true)]
pub enum AppCommand {
	/// Switch to another network. [Polkadot / Kusama/ Darwinia /Crab | Pangolin / Pangoro]
	#[command(subcommand)]
	SwitchNetwork(Network),
	/// RPC interfaces.
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
	Runtime(RuntimeCommand),
	/// Print usage
	Usage,
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
	/// Print usage
	Usage,
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "chain")]
#[allow(clippy::enum_variant_names)]
pub enum ChainCommand {
	/// Get the chain block
	GetBlock {
		#[arg(value_name = "HASH", long)]
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
		#[arg(value_name = "HASH", long)]
		hash: String,
	},
	/// Print usage
	Usage,
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "state")]
pub enum StateCommand {
	RuntimeVersion {
		#[arg(value_name = "HASH", long)]
		hash: Option<String>,
	},
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "account-info")]
pub enum AccountInfoCommand {
	/// Get the account balance
	Balances {
		#[arg(name = "account-id", value_name = "ACCOUNT_ID", long)]
		account_id: String,
		#[arg(name = "at-block", help = "which block", long)]
		at_block: Option<String>,
	},
	/// Get the account nonce
	Nonce {
		#[arg(name = "account-id", value_name = "ACCOUNT_ID", long)]
		account_id: String,
		#[arg(name = "at-block", long)]
		at_block: Option<String>,
	},
	/// Print usage
	Usage,
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "pallets")]
#[allow(clippy::enum_variant_names)]
pub enum RuntimeCommand {
	/// List all pallets in this chain
	ListPallets,
	/// List storages in certain pallet
	ListPalletStorages {
		#[arg(name = "pallet-name", long)]
		pallet_name: String,
	},
	/// List constants in certain pallet
	ListPalletConstants {
		#[arg(name = "pallet-name", long)]
		pallet_name: String,
	},
	/// Print usage
	Usage,
}
