// std
use std::io;
// crates.io
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use ratatui::{
	prelude::{
		text::Line, Backend, Color, Constraint, Direction, Frame, Layout, Rect, Span, Style,
		Terminal,
	},
	style::Stylize,
	widgets::*,
};
// this crate
use crate::{
	networks::ChainInfo,
	rpc::{RpcClient, SystemPaneInfo},
};

pub(crate) struct DashBoard<'a, CI> {
	#[allow(dead_code)]
	pub client: &'a RpcClient<CI>,
	pub system_pane_info: SystemPaneInfo,
	pub tab_titles: Vec<&'a str>,
	pub index: usize,
}

impl<'a, CI: ChainInfo> DashBoard<'a, CI> {
	pub(crate) fn new(
		client: &'a RpcClient<CI>,
		system_pane_info: SystemPaneInfo,
	) -> DashBoard<'a, CI> {
		DashBoard {
			client,
			system_pane_info,
			tab_titles: vec!["Blocks", "Transactions", "Events"],
			index: 0,
		}
	}

	pub fn next(&mut self) {
		self.index = (self.index + 1) % self.tab_titles.len();
	}

	pub fn previous(&mut self) {
		if self.index > 0 {
			self.index -= 1;
		} else {
			self.index = self.tab_titles.len() - 1;
		}
	}
}

pub(crate) fn run_dashboard<B, CI>(
	terminal: &mut Terminal<B>,
	mut app: DashBoard<CI>,
) -> io::Result<()>
where
	B: Backend,
	CI: ChainInfo,
{
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

pub(crate) fn ui<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>)
where
	B: Backend,
	CI: ChainInfo,
{
	let size = f.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
		.split(size);

	draw_system(f, app, chunks[0]);
	draw_tabs(f, app, chunks[1]);
}

fn draw_system<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![Constraint::Percentage(100)])
		.split(area);

	let spec_version = app.system_pane_info.runtime_version.spec_version.to_string();
	let rows = vec![
		Row::new(vec!["SystemName    =>", app.system_pane_info.system_name.as_str()]),
		Row::new(vec!["SystemVersion =>", app.system_pane_info.system_version.as_str()]),
		Row::new(vec!["ChainName     =>", app.system_pane_info.chain_name.as_str()]),
		Row::new(vec!["ChainType     =>", app.system_pane_info.chain_type.as_str()]),
		Row::new(vec!["TokenSymbol   =>", app.system_pane_info.token_symbol.as_str()]),
		Row::new(vec!["TokenDecimal  =>", app.system_pane_info.token_decimal.as_str()]),
		Row::new(vec!["SpecName      =>", &app.system_pane_info.runtime_version.spec_name]),
		Row::new(vec!["ImplName      =>", &app.system_pane_info.runtime_version.impl_name]),
		Row::new(vec!["SpecVersion   =>", spec_version.as_str()]),
	];

	let table = Table::new(rows)
		.block(Block::default().title("System Information").borders(Borders::ALL))
		.header(Row::new(vec!["ITEM", "VALUE"]).style(Style::default().fg(Color::Cyan)).bold())
		.style(Style::default().fg(Color::Yellow))
		.column_spacing(1)
		.widths(&[Constraint::Length(20), Constraint::Length(20)]);
	f.render_widget(table, chunks[0]);
}

fn draw_tabs<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.split(area);
	let titles = app
		.tab_titles
		.iter()
		.map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::Yellow).bold())))
		.collect();
	let tabs = Tabs::new(titles)
		.block(Block::default().borders(Borders::ALL).title("Chain Data"))
		.select(app.index)
		.style(Style::default().fg(Color::Yellow))
		.highlight_style(Style::default().fg(Color::Cyan));
	f.render_widget(tabs, chunks[0]);

	match app.index {
		0 => draw_blocks_tab(f, app, chunks[1]),
		1 => draw_transactions_tab(f, app, chunks[1]),
		2 => draw_events_tab(f, app, chunks[1]),
		_ => {},
	};
}

fn draw_blocks_tab<B, CI>(f: &mut Frame<B>, _app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let text = vec![Line::from("Block Page")];
	let paragraph = Paragraph::new(text);
	f.render_widget(paragraph, area);
}
fn draw_transactions_tab<B, CI>(f: &mut Frame<B>, _app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let text = vec![Line::from("Transaction Page")];
	let paragraph = Paragraph::new(text);
	f.render_widget(paragraph, area);
}

fn draw_events_tab<B, CI>(f: &mut Frame<B>, _app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let text = vec![Line::from("Event Page")];
	let paragraph = Paragraph::new(text);
	f.render_widget(paragraph, area);
}
