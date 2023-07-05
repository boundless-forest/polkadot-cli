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
	// TODO: ADD HELP COMMAND
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "rpc")]
pub enum RpcCommand {
	/// Get RPC methods
	RpcMethods,
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
	/// Get block by hash
	ChainBlockByHash { hash: String, number: u8 },
	/// Get block header
	ChainHeader { hash: String },
}

#[derive(Subcommand, Clone, Debug)]
pub enum Network {
	Local,
	Polkadot,
	Kusama,
}
