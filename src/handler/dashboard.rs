// std
use std::io;
// crates.io
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use ratatui::{
	prelude::{
		Backend, Color, Constraint, Direction, Frame, Layout, Line, Modifier, Style, Terminal,
	},
	style::Stylize,
	widgets::*,
};

pub(crate) struct DashBoard<'a> {
	pub titles: Vec<&'a str>,
	pub index: usize,
}

impl<'a> DashBoard<'a> {
	pub(crate) fn new() -> DashBoard<'a> {
		DashBoard { titles: vec!["Tab0", "Tab1", "Tab2", "Tab3"], index: 0 }
	}

	pub fn next(&mut self) {
		self.index = (self.index + 1) % self.titles.len();
	}

	pub fn previous(&mut self) {
		if self.index > 0 {
			self.index -= 1;
		} else {
			self.index = self.titles.len() - 1;
		}
	}
}

pub(crate) fn run_dashboard<B: Backend>(
	terminal: &mut Terminal<B>,
	mut app: DashBoard,
) -> io::Result<()> {
	loop {
		terminal.draw(|f| ui(f, &app))?;

		if let Event::Key(key) = read()? {
			if key.kind == KeyEventKind::Press {
				match key.code {
					KeyCode::Char('q') => return Ok(()),
					KeyCode::Right => app.next(),
					KeyCode::Left => app.previous(),
					_ => {},
				}
			}
		}
	}
}

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, app: &DashBoard) {
	let size = f.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.split(size);

	let block = Block::default().on_white().black();
	f.render_widget(block, size);
	let titles = app
		.titles
		.iter()
		.map(|t| {
			let (first, rest) = t.split_at(1);
			Line::from(vec![first.gray(), rest.gray()])
		})
		.collect();
	let tabs = Tabs::new(titles)
		.block(Block::default().borders(Borders::ALL).title("Tabs"))
		.select(app.index)
		.style(Style::default().fg(Color::Cyan))
		.highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Black));
	f.render_widget(tabs, chunks[0]);
	let inner = match app.index {
		0 => Block::default().title("Inner 0").borders(Borders::ALL),
		1 => Block::default().title("Inner 1").borders(Borders::ALL),
		2 => Block::default().title("Inner 2").borders(Borders::ALL),
		3 => Block::default().title("Inner 3").borders(Borders::ALL),
		_ => unreachable!(),
	};
	f.render_widget(inner, chunks[1]);
}
