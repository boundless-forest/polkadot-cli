// std
use std::io;
// crates.io
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use ratatui::{
	prelude::{
		text, Backend, Color, Constraint, Direction, Frame, Layout, Modifier, Rect, Span, Style,
		Terminal,
	},
	widgets::*,
};

pub(crate) struct DashBoard<'a> {
	pub titles: Vec<&'a str>,
	pub index: usize,
}

impl<'a> DashBoard<'a> {
	pub(crate) fn new() -> DashBoard<'a> {
		DashBoard { titles: vec!["Blocks", "Transactions", "Events"], index: 0 }
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
		terminal.draw(|f| ui(f, &mut app))?;

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

pub(crate) fn ui<B: Backend>(f: &mut Frame<B>, app: &mut DashBoard) {
	let size = f.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
		.split(size);

	draw_system(f, app, chunks[0]);
	draw_tabs(f, app, chunks[1]);
}

fn draw_tabs<B>(f: &mut Frame<B>, app: &mut DashBoard, area: Rect)
where
	B: Backend,
{
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.split(area);
	let titles = app
		.titles
		.iter()
		.map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
		.collect();
	let tabs = Tabs::new(titles)
		.block(Block::default().borders(Borders::ALL).title("DashBoard"))
		.select(app.index)
		.style(Style::default().fg(Color::Cyan))
		.highlight_style(Style::default().fg(Color::Yellow));
	f.render_widget(tabs, chunks[0]);
	match app.index {
		0 => draw_blocks_tab(f, app, chunks[1]),
		1 => draw_transactions_tab(f, app, chunks[1]),
		2 => draw_events_tab(f, app, chunks[1]),
		_ => {},
	};
}

fn draw_system<B>(f: &mut Frame<B>, app: &mut DashBoard, area: Rect)
where
	B: Backend,
{
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![Constraint::Percentage(100)])
		.split(area);
	let configs = ["RuntimeVersion", "SystemVersion"];
	let items: Vec<Row> = configs
		.iter()
		.map(|c| {
			let cells = vec![
				Cell::from(Span::raw(format!("{c:?}: "))),
				Cell::from(Span::styled("TODO", Style::default())),
				Cell::from(Span::styled("TODO", Style::default())),
			];
			Row::new(cells)
		})
		.collect();
	let table = Table::new(items)
		.block(Block::default().title("System").borders(Borders::ALL))
		.widths(&[
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(10),
        ]);
	f.render_widget(table, chunks[0]);
}
fn draw_blocks_tab<B>(f: &mut Frame<B>, app: &mut DashBoard, area: Rect)
where
	B: Backend,
{
	let text = vec![text::Line::from(
		"This is a paragraph with several lines. You can change style your text the way you want",
	)];
	let block = Block::default().borders(Borders::ALL).title(Span::styled(
		"Footer1",
		Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
	));
	let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
fn draw_transactions_tab<B>(f: &mut Frame<B>, app: &mut DashBoard, area: Rect)
where
	B: Backend,
{
	let text = vec![text::Line::from(
		"This is a paragraph with several lines. You can change style your text the way you want",
	)];
	let block = Block::default().borders(Borders::ALL).title(Span::styled(
		"Footer2",
		Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
	));
	let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
fn draw_events_tab<B>(f: &mut Frame<B>, app: &mut DashBoard, area: Rect)
where
	B: Backend,
{
	let text = vec![text::Line::from(
		"This is a paragraph with several lines. You can change style your text the way you want",
	)];
	let block = Block::default().borders(Borders::ALL).title(Span::styled(
		"Footer3",
		Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
	));
	let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
