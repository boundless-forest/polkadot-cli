use crate::command::AppCommand;

pub fn handle_commands(command: AppCommand) -> anyhow::Result<()> {
	match command {
		AppCommand::SwitchNetwork(network) => {
			println!("Missing switch network implementation");
		},
		AppCommand::Rpc(rpc_commands) => {
			println!("Missing RPC network implementation");
		},
		_ => todo!(),
	}

	Ok(())
}
