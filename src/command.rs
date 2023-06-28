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
	/// Transfer native token
	Transfer,
	/// Query the balance of an account
	Balance,
	/// Query the account details
	Account,
	/// Query the storage item
	Storage,
}

#[derive(Subcommand, Clone, Debug)]
pub enum RpcCommand {
	/// Get block by hash
	GetBlockByHash,
	/// Get block by number
	GetBlockHash,
	/// Get block header
	GetHeader,
	/// Get RPC methods
	RpcMethods,
	// Get System chain information
	SysChain,
	/// Get System chain type
	SysChainType,
	/// Get System health
	SysHealth,
	/// Get System local peer id
	SysLocalPeerId,
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
