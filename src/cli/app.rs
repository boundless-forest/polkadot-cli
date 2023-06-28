use clap::{Parser, Subcommand};

/// Substrate-based chain's cli client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Switch network, default is local
    SwitchNetwork,
    /// RPC interfaces
	Rpc,
    /// Transfer native token
    Transfer,
    /// Query the balance of an account
    Balance,
    /// Query the account details
    Account,
    /// Query the storage item
    Storage
}
