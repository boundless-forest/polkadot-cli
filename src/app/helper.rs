// std
use std::{
	borrow::Cow::{self, Owned},
	fs,
	fs::File,
	io::Write,
	iter,
	path::PathBuf,
};
// crates.io
use clap::Command;
use colored::Colorize;
use figlet_rs::FIGfont;
use rustyline::{
	completion::{extract_word, Completer, Pair},
	highlight::Highlighter,
	hint::{Hinter, HistoryHinter},
	history::FileHistory,
	validate::Validator,
	ColorMode, CompletionType, Context, Editor, Helper,
};
use serde::{Deserialize, Serialize};
// this crate
use super::command::AppCommand;
use crate::{errors::AppError, networks::Network};

/// The configuration of the App
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
	#[serde(default)]
	pub network: Network,
}

const ESCAPE_CHAR: Option<char> = Some('\\');
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

pub fn print_info() {
	let standard_font = FIGfont::standard().unwrap();
	let figure = standard_font.convert("suber");
	if let Some(figure) = figure {
		println!("{}", figure);
		println!(
			"{}",
			"This is the one-stop substrate command-line tool, the command-line version of the Polkadot Apps."
				.green()
				.bold()
				.italic()
		);
	}
}

pub fn load_history() -> Result<PathBuf, AppError> {
	let mut history_file_dir = dirs::home_dir().unwrap();
	history_file_dir.push(".suber");
	if !history_file_dir.exists() {
		fs::create_dir(history_file_dir.clone()).map_err(|e| AppError::Custom(e.to_string()))?;
	}
	let history_file = history_file_dir.join("history");
	if !history_file.is_file() {
		let _ = File::create(history_file.clone());
	}
	Ok(history_file)
}

pub fn this_crate_editor() -> Editor<CommandHelper<HistoryHinter>, FileHistory> {
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

pub struct CommandHelper<H> {
	hinter: H,
	command: Command,
	config: Config,
}

impl<H> CommandHelper<H> {
	/// CommandHelper constructor
	fn new(hinter: H) -> Self {
		let init = Command::new("suber")
			.subcommand_required(true)
			.arg_required_else_help(true)
			.no_binary_name(true);
		let command = <AppCommand as clap::Subcommand>::augment_subcommands(init);
		Self { hinter, command, config: Config { network: Network::default() } }
	}

	fn prefix_command<'s, I: Iterator<Item = &'s str>>(
		&self,
		command: &Command,
		mut prefixes: iter::Peekable<I>,
	) -> Option<Command> {
		if let Some(prefix) = prefixes.next() {
			for subcommand in command.get_subcommands() {
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
			Some(command.clone())
		} else {
			None
		}
	}

	/// Load the config from file
	pub fn load_config(&mut self) -> Result<Config, AppError> {
		let mut config_dir = dirs::home_dir().unwrap();
		config_dir.push(".suber");
		if !config_dir.exists() {
			fs::create_dir(config_dir.clone()).map_err(|e| AppError::Custom(e.to_string()))?;
		}
		let config_file = config_dir.join("config.json");
		if !config_file.is_file() {
			let mut file = File::create(config_file.clone()).unwrap();

			let config = Config::default();
			let json_config = serde_json::to_string_pretty(&config).unwrap();
			file.write_all(json_config.as_bytes()).unwrap();
			return Ok(config);
		}
		let file = File::open(config_file).unwrap();
		let config: Config = serde_json::from_reader(file).unwrap();

		self.config = config.clone();

		Ok(config)
	}

	/// Save the config to file
	pub fn save_config(&mut self, config: Config) -> Result<(), AppError> {
		let mut config_dir = dirs::home_dir().unwrap();
		config_dir.push(".suber");
		let config_file = config_dir.join("config.json");

		let mut file = File::create(config_file).unwrap();
		let json_config = serde_json::to_string_pretty(&config).unwrap();
		file.write_all(json_config.as_bytes()).unwrap();
		return Ok(());
	}

	/// Get the config
	pub fn config(&self) -> Config {
		self.config.clone()
	}
}

impl<H: Hinter> Helper for CommandHelper<H> {}
impl<H: Hinter> Highlighter for CommandHelper<H> {
	fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
		Owned(hint.bright_yellow().to_string())
	}

	fn highlight_candidate<'c>(
		&self,
		candidate: &'c str,
		_completion: CompletionType,
	) -> Cow<'c, str> {
		Owned(candidate.bright_yellow().to_string())
	}
}
impl<H: Hinter> Validator for CommandHelper<H> {}

impl<H: Hinter> Hinter for CommandHelper<H> {
	type Hint = H::Hint;

	fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<H::Hint> {
		self.hinter.hint(line, pos, ctx)
	}
}

impl<H: Hinter> Completer for CommandHelper<H> {
	type Candidate = Pair;

	fn complete(
		&self,
		line: &str,
		pos: usize,
		_ctx: &Context<'_>,
	) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
		println!("");
		// println!("=====> Completing: line: {}, pos: {}", line, pos);
		let (start, mut word) = extract_word(line, pos, ESCAPE_CHAR, default_break_chars);
		// println!("=====> Completing: start: {}, word: {}", start, word);

		let prefixes = shell_words::split(&line[..pos]).unwrap();
		// println!("=====> Completing: prefix: {:?}", prefixes);

		let mut candidates = Vec::new();
		if let Some(command) =
			self.prefix_command(&self.command, prefixes.iter().map(String::as_str).peekable())
		{
			candidates = command
				.get_subcommands()
				.cloned()
				.map(|i| Pair {
					display: i.get_name().to_owned(),
					replacement: i.get_name().to_owned(),
				})
				.filter(|c| c.display.starts_with(&word) && word != command.get_name())
				.collect();

			if candidates.is_empty() {
				if !word.starts_with("-") || !word.starts_with("--") {
					word = "--";
				}
				// println!(
				// 	"=====> Completing: no candidates, args: {:?}",
				// 	command
				// 		.get_arguments()
				// 		.cloned()
				// 		.map(|i| i.get_id().to_string())
				// 		.collect::<Vec<String>>()
				// );
				candidates = command
					.get_arguments()
					.cloned()
					.map(|i| Pair {
						display: format!("--{}", i.get_id().to_string()),
						replacement: format!("--{}", i.get_id().to_string()),
					})
					.filter(|c| !prefixes.contains(&c.display))
					.filter(|c| c.display.starts_with(&word))
					.collect();
			}
		}
		// println!(
		// 	"=====> Completing: candidates: {:?}",
		// 	candidates
		// 		.iter()
		// 		.map(|i| (i.clone().display, i.clone().replacement))
		// 		.collect::<Vec<(String, String)>>()
		// );

		Ok((start, candidates))
	}
}
