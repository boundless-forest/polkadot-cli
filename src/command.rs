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
}

#[derive(Subcommand, Clone, Debug)]
#[command(name = "rpc")]
pub enum RpcCommand {
	/// Get block by hash
	GetBlockByHash {
		#[arg(short, long)]
		hash: String,
	},
	/// Get block header
	GetHeader {
		#[arg(short, long)]
		hash: String,
	},
	/// Get RPC methods
	RpcMethods,
	/// Get System Name
	SysName,
	/// Get System Properties
	SysProperties,
	/// Get System Version
	SysVersion,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Network {
	Local,
	Polkadot,
	Kusama,
}
