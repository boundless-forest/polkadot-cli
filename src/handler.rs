use crate::command::AppCommand;
use colored::Colorize;

pub fn handle_commands(command: AppCommand) -> anyhow::Result<()> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			println!("Missing switch network implementation");
		},
		AppCommand::Rpc(rpc_commands) => {
			println!("Missing RPC network implementation");
		},
		_ => {
			println!("{}", "Invalid command, please check your command and input params".red());
		},
	}

	Ok(())
}
