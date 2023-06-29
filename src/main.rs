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
	history::{self, FileHistory},
	validate::Validator,
	ColorMode, CompletionType, Context, Editor, Helper,
};
use std::{fs, fs::File, path::PathBuf};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut editor = this_crate_editor();
	let history_file = load_history()?;
	editor.load_history(&history_file)?;

	loop {
		let prompt = editor.readline("suber >> ");
		match prompt {
			Ok(prompt) => {
				let command = AppCommand::parse_from(prompt.split_whitespace());
				handler::handle_commands(command)?;
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

	editor.save_history(&history_file)?;
	Ok(())
}

pub(crate) fn load_history() -> anyhow::Result<PathBuf> {
	let mut history_file_dir = dirs::home_dir().unwrap();
	history_file_dir.push(".suber");
	if !history_file_dir.exists() {
		fs::create_dir(history_file_dir.clone())?;
	}
	let history_file = history_file_dir.join("history");
	if !history_file.is_file() {
		let _ = File::create(history_file.clone())?;
	}
	Ok(history_file)
}

pub(crate) fn this_crate_editor() -> Editor<CommandHelper<HistoryHinter>, FileHistory> {
	let config_builder = rustyline::Config::builder();
	let config = config_builder
		.auto_add_history(true)
		.history_ignore_space(true)
		.color_mode(ColorMode::Enabled)
		.completion_type(CompletionType::List)
		.build();

	let editor_helper = CommandHelper::new(HistoryHinter {});

	let mut editor = Editor::with_history(config, FileHistory::new()).unwrap();
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
