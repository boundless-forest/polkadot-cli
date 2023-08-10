// this crate
use crate::Network;
// crates.io
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
#[clap(no_binary_name = true)]
pub enum AppCommand {
	/// Switch to another network. [Polkadot | Kusama | Darwinia | Crab | Pangolin | Pangoro]
	#[command(subcommand)]
	SwitchNetwork(Network),
	/// RPC commands, e.g sys-name, sys-version, chain-type, health.
	#[command(subcommand)]
	Rpc(RpcCommand),
	/// Chain commands, e.g get-block, get-block-hash, get-finalized-head, get-finalized-number.
	#[command(subcommand)]
	Chain(ChainCommand),
	/// Account commands, e.g balance, nonce.
	#[command(subcommand)]
	AccountInfo(AccountInfoCommand),
	/// Runtime commands, e.g list-pallets, list-pallet-storages, list-runtime-storages.
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
	/// Get the chain block by hash
	GetBlock {
		#[arg(value_name = "HASH", long)]
		hash: String,
	},
	/// Get the block hash by number
	GetBlockHash {
		#[arg(long)]
		number: u32,
	},
	/// Get the finalized head hash
	GetFinalizedHead,
	/// Get the finalized head number
	GetFinalizedNumber,
	/// Get the header for a specific block by hash
	GetHeader {
		#[arg(value_name = "HASH", long)]
		hash: String,
	},
	/// Print usage
	Usage,
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
	/// List storages of the certain pallet
	ListPalletStorages {
		#[arg(name = "pallet-name", long)]
		pallet_name: String,
	},
	/// List constants of the certain pallet
	ListPalletConstants {
		#[arg(name = "pallet-name", long)]
		pallet_name: String,
	},
	/// Get the runtime version
	RuntimeVersion {
		#[arg(value_name = "HASH", long)]
		hash: Option<String>,
	},
	/// Print usage
	Usage,
}
