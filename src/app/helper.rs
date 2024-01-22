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

pub const POLKADOT_CLI: &str = "polkadot-cli";

/// The configuration of the App
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
	#[serde(default)]
	pub network: Network,
}

const ESCAPE_CHAR: Option<char> = Some('\\');
const fn default_break_chars(c: char) -> bool {
	matches!(c, ' ' | '\t' | '\n' | '"' | '\\' | '\'' | '`' | '@' | '$' | '>' | '<' | '=' | ';' | '|' | '&' | '{' | '(' | '\0')
}

/// Create a line editor.
pub fn create_editor() -> Editor<EditorHelper<HistoryHinter>, FileHistory> {
	let config_builder = rustyline::Config::builder();
	let config = config_builder
		.auto_add_history(true)
		.history_ignore_space(true)
		.color_mode(ColorMode::Enabled)
		.completion_type(CompletionType::Fuzzy)
		.build();

	let editor_helper = EditorHelper::new(HistoryHinter {});
	let mut editor = Editor::with_history(config, FileHistory::new()).unwrap();
	editor.set_helper(Some(editor_helper));
	editor
}

/// Return the APP command history file path.
pub fn history_path() -> Result<PathBuf, AppError> {
	let app_path = app_root_path()?;
	let history_file = app_path.join("history");
	if !history_file.is_file() {
		let _ = File::create(history_file.clone());
	}
	Ok(history_file)
}

/// App Editor helper
pub struct EditorHelper<H> {
	hinter: H,
	command: Command,
	config: Config,
}

impl<H> EditorHelper<H> {
	/// EditorHelper constructor
	fn new(hinter: H) -> Self {
		let init = Command::new(POLKADOT_CLI)
			.disable_help_flag(true)
			.disable_help_subcommand(true)
			.no_binary_name(true);
		let command = <AppCommand as clap::Subcommand>::augment_subcommands(init);
		Self { hinter, command, config: Config { network: Network::default() } }
	}

	/// Load the config from file
	pub fn load_config(&mut self) -> Result<Config, AppError> {
		let app_path = app_root_path()?;
		let config_file = app_path.join("config.json");

		if !config_file.is_file() {
			let mut file = File::create(config_file).unwrap();
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
		let app_path = app_root_path()?;
		let config_file = app_path.join("config.json");

		let mut file = File::create(config_file).unwrap();
		let json_config = serde_json::to_string_pretty(&config).unwrap();
		file.write_all(json_config.as_bytes()).unwrap();
		self.config = config;
		Ok(())
	}

	/// Get the config
	pub fn config(&self) -> Config {
		self.config.clone()
	}
}

impl<H: Hinter> Helper for EditorHelper<H> {}
impl<H: Hinter> Highlighter for EditorHelper<H> {
	fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
		Owned(format!("\x1b[1m{hint}\x1b[m"))
	}

	fn highlight_candidate<'c>(&self, candidate: &'c str, _completion: CompletionType) -> Cow<'c, str> {
		Owned(candidate.to_string())
	}
}
impl<H: Hinter> Validator for EditorHelper<H> {}

impl<H: Hinter> Hinter for EditorHelper<H> {
	type Hint = H::Hint;

	fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<H::Hint> {
		self.hinter.hint(line, pos, ctx)
	}
}

impl<H: Hinter> Completer for EditorHelper<H> {
	type Candidate = Pair;

	fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
		println!();
		let (start, mut word) = extract_word(line, pos, ESCAPE_CHAR, default_break_chars);
		let prefixes = shell_words::split(&line[..pos]).unwrap();

		let mut candidates = Vec::new();
		if let Some(command) = prefix_command(&self.command, prefixes.iter().map(String::as_str).peekable()) {
			candidates = command
				.get_subcommands()
				.cloned()
				.map(|i| Pair { display: i.get_name().to_owned(), replacement: i.get_name().to_owned() })
				.filter(|c| c.display.starts_with(word) && word != command.get_name())
				.collect();

			if candidates.is_empty() {
				if !word.starts_with('-') || !word.starts_with("--") {
					word = "--";
				}
				candidates = command
					.get_arguments()
					.cloned()
					.map(|i| Pair {
						display: format!("--{}", i.get_id().to_string()),
						replacement: format!("--{}", i.get_id().to_string()),
					})
					.filter(|c| !prefixes.contains(&c.display))
					.filter(|c| c.display.starts_with(word))
					.collect();
			}
		}

		Ok((start, candidates))
	}
}

fn prefix_command<'s, I: Iterator<Item = &'s str>>(command: &Command, mut prefixes: iter::Peekable<I>) -> Option<Command> {
	if let Some(prefix) = prefixes.next() {
		for subcommand in command.get_subcommands() {
			if subcommand.get_name() == prefix
				|| subcommand.get_display_name().unwrap_or_default() == prefix
				|| subcommand.get_all_aliases().any(|s| s == prefix)
			{
				return if prefixes.peek().is_none() { Some(subcommand.clone()) } else { prefix_command(subcommand, prefixes) };
			}
		}
	}

	if prefixes.peek().is_none() || !command.has_subcommands() {
		Some(command.clone())
	} else {
		None
	}
}

pub fn app_root_path() -> Result<PathBuf, AppError> {
	let mut root = dirs::home_dir().unwrap();
	root.push(format!(".{}", POLKADOT_CLI));

	if !root.exists() {
		fs::create_dir(root.clone()).map_err(|e| AppError::Custom(e.to_string()))?;

		let metadata_path = root.join("metadata");
		fs::create_dir(metadata_path).expect("Failed to create metadata directory");
	}
	Ok(root)
}

pub fn metadata_path() -> Result<PathBuf, AppError> {
	let path = app_root_path().expect("Failed to get app root path").join("metadata");
	Ok(path)
}

/// Print the app welcome message.
pub fn print_welcome_message() {
	const INTRODUCTION: &str = "This is the all-in-one substrate command assistant, the Polkadot Apps CLI edition.";
	const USAGE: &str = "
Tips:
- `usage` to ask help.
- `Tab` to complete.
- `Ctrl + c` to quit.
";

	const UNIVERS_FONT: &[u8] = include_bytes!("../resources/univers.flf");
	let font = FIGfont::from_content(&String::from_utf8_lossy(UNIVERS_FONT)).unwrap();
	let figure = font.convert(POLKADOT_CLI);
	if let Some(figure) = figure {
		println!("{}", figure);
		println!("{}", INTRODUCTION.bright_purple().bold().italic());
		println!("{}", USAGE.bright_purple().italic());
	}
}
