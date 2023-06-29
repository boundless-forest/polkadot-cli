mod command;
mod errors;
mod handler;
mod rpc;

// 1. Setup node: ./target/release/node-template --dev --tmp --rpc-methods=Unsafe --rpc-cors all
// --rpc-external --ws-external 2: ws://192.168.31.52:9944

use crate::command::AppCommand;
use clap::{Command, Parser, Subcommand};
use colored::Colorize;
use figlet_rs::FIGfont;
use rustyline::{
	completion::{extract_word, Completer, Pair},
	error::ReadlineError,
	highlight::Highlighter,
	hint::{Hinter, HistoryHinter},
	history::{self, FileHistory},
	validate::Validator,
	ColorMode, CompletionType, Context, Editor, Helper,
};
use std::{fs, fs::File, iter, path::PathBuf};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let mut editor = this_crate_editor();
	let history_file = load_history()?;
	editor.load_history(&history_file)?;

	print_info();
	loop {
		let prompt = editor.readline(&"suber >> ".green().bold().to_string());
		match prompt {
			Ok(prompt) => {
				if let Ok(command) = AppCommand::try_parse_from(prompt.split_whitespace()) {
					handler::handle_commands(command)?;
				} else {
					println!("Invalid prompt: {}", prompt);
				}
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

pub(crate) fn print_info() {
	let standard_font = FIGfont::standard().unwrap();
	let figure = standard_font.convert("suber");
	if let Some(figure) = figure {
		println!("{}", figure);
	}
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
	command: Command,
}

impl<H> CommandHelper<H> {
	fn new(hinter: H) -> Self {
		let init = Command::new("suber").subcommand_required(true).arg_required_else_help(true);
		let command = <AppCommand as clap::Subcommand>::augment_subcommands(init);
		Self { hinter, command }
	}

	fn prefix_command<'s, I: Iterator<Item = &'s str>>(
		&self,
		command: &Command,
		mut prefixes: iter::Peekable<I>,
	) -> Option<Command> {
		if let Some(prefix) = prefixes.next() {
			for subcommand in self.command.get_subcommands() {
				if subcommand.get_name() == prefix
					|| subcommand.get_display_name().unwrap_or_default() == prefix
					|| subcommand.get_all_aliases().into_iter().any(|s| s == prefix)
				{
					return if prefixes.peek().is_none() {
						Some(subcommand.clone())
					} else {
						self.prefix_command(subcommand, prefixes)
					};
				}
			}
		}

		if prefixes.peek().is_none() || !command.has_subcommands() {
			Some(self.command.clone())
		} else {
			None
		}
	}
}

impl<H: Hinter> Helper for CommandHelper<H> {}
impl<H: Hinter> Highlighter for CommandHelper<H> {}
impl<H: Hinter> Validator for CommandHelper<H> {}

impl<H: Hinter> Hinter for CommandHelper<H> {
	type Hint = H::Hint;

	fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<H::Hint> {
		self.hinter.hint(line, pos, ctx)
	}
}

const fn default_break_chars(c: char) -> bool {
	matches!(
		c,
		' ' | '\t'
			| '\n' | '"' | '\\'
			| '\'' | '`' | '@'
			| '$' | '>' | '<'
			| '=' | ';' | '|'
			| '&' | '{' | '('
			| '\0'
	)
}
const ESCAPE_CHAR: Option<char> = Some('\\');

impl<H: Hinter> Completer for CommandHelper<H> {
	type Candidate = Pair;

	fn complete(
		&self,
		line: &str,
		pos: usize,
		ctx: &Context<'_>,
	) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
		println!("");
		println!("=====> Completing: line: {}, pos: {}", line, pos);
		let (start, word) = extract_word(line, pos, ESCAPE_CHAR, default_break_chars);
		println!("=====> Completing: start: {}, word: {}", start, word);

		let prefixes = shell_words::split(&line[..pos]).unwrap();
		println!("=====> Completing: prefix: {:?}", prefixes);

		let command =
			self.prefix_command(&self.command, prefixes.iter().map(String::as_str).peekable());

		Ok((pos, Vec::new()))
	}
}
