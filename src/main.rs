mod command;
mod errors;
mod handler;
mod rpc;

// 1. Setup node: ./target/release/node-template --dev --tmp --rpc-methods=Unsafe --rpc-cors all
// --rpc-external --ws-external 2: ws://192.168.31.52:9944

use crate::command::AppCommand;
use clap::Parser;
use rustyline::{
	completion::Completer,
	error::ReadlineError,
	highlight::Highlighter,
	hint::{Hinter, HistoryHinter},
	history::MemHistory,
	validate::Validator,
	ColorMode, CompletionType, Context, Editor, Helper,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut editor = this_crate_editor();
	loop {
		let prompt = editor.readline("suber >> ");
		match prompt {
			Ok(prompt) => {
				let command = AppCommand::parse_from(prompt.split_whitespace());
				handler::handle_commands(command)?;
				// rl.add_history_entry(line.as_str());
			},
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			},
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			},
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			},
		}
	}

	Ok(())
}

pub(crate) fn this_crate_editor() -> Editor<CommandHelper<HistoryHinter>, MemHistory> {
	let config_builder = rustyline::Config::builder();
	let config = config_builder
		.auto_add_history(true)
		.color_mode(ColorMode::Enabled)
		.history_ignore_space(true)
		.completion_type(CompletionType::List)
		.build();

	let editor_helper = CommandHelper::new(HistoryHinter {});

	let mut editor = Editor::with_history(config, MemHistory::new()).unwrap();
	editor.set_helper(Some(editor_helper));
	editor
}

struct CommandHelper<H> {
	hinter: H,
}

impl<H> CommandHelper<H> {
	fn new(hinter: H) -> Self {
		Self { hinter }
	}
}

impl<H: Hinter> Helper for CommandHelper<H> {}
impl<H: Hinter> Hinter for CommandHelper<H> {
	type Hint = H::Hint;

	fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<H::Hint> {
		self.hinter.hint(line, pos, ctx)
	}
}
impl<H: Hinter> Highlighter for CommandHelper<H> {}
impl<H: Hinter> Completer for CommandHelper<H> {
	type Candidate = String;
}
impl<H: Hinter> Validator for CommandHelper<H> {}
