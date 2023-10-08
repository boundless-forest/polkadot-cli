// std
use std::{collections::VecDeque, io, sync::Arc};
// crates.io
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use ratatui::{
	prelude::{
		text::Line, Backend, Color, Constraint, Direction, Frame, Layout, Modifier, Rect, Span,
		Style, Terminal,
	},
	style::Stylize,
	widgets::*,
};
use sp_runtime::traits::Header;
use tokio::sync::mpsc::UnboundedReceiver;
// this crate
use crate::{
	networks::ChainInfo,
	rpc::{HeaderForChain, RpcClient, SystemPaneInfo},
};

const BLOCKS_MAX_LIMIT: usize = 20;

pub(crate) struct DashBoard<CI: ChainInfo> {
	#[allow(dead_code)]
	pub client: Arc<RpcClient<CI>>,
	pub system_pane_info: SystemPaneInfo,
	pub blocks_rev: UnboundedReceiver<HeaderForChain<CI>>,
	pub block_headers: StatefulList<HeaderForChain<CI>>,
	pub tab_titles: Vec<String>,
	pub index: usize,
}

impl<CI: ChainInfo> DashBoard<CI> {
	pub(crate) fn new(
		client: Arc<RpcClient<CI>>,
		system_pane_info: SystemPaneInfo,
		blocks_rev: UnboundedReceiver<HeaderForChain<CI>>,
	) -> DashBoard<CI> {
		DashBoard {
			client,
			system_pane_info,
			blocks_rev,
			block_headers: StatefulList::with_items(VecDeque::with_capacity(BLOCKS_MAX_LIMIT)),
			tab_titles: vec![
				String::from("Blocks"),
				String::from("Transactions"),
				String::from("Events"),
			],
			index: 0,
		}
	}

	pub fn next_tab(&mut self) {
		self.index = (self.index + 1) % self.tab_titles.len();
	}

	pub fn previous_tab(&mut self) {
		if self.index > 0 {
			self.index -= 1;
		} else {
			self.index = self.tab_titles.len() - 1;
		}
	}

	pub fn previous_block(&mut self) {
		self.block_headers.previous();
	}

	pub fn next_block(&mut self) {
		self.block_headers.next();
	}
}

pub(crate) async fn run_dashboard<B, CI>(
	terminal: &mut Terminal<B>,
	mut app: DashBoard<CI>,
) -> io::Result<()>
where
	B: Backend,
	CI: ChainInfo,
{
	loop {
		terminal.draw(|f| ui(f, &mut app))?;

		if let Ok(header) = app.blocks_rev.try_recv() {
			if app.block_headers.items.len() == app.block_headers.items.capacity() {
				app.block_headers.items.pop_front();
			}
			app.block_headers.items.push_back(header);
		}

		if let Event::Key(key) = read()? {
			if key.kind == KeyEventKind::Press {
				match key.code {
					KeyCode::Char('q') => return Ok(()),
					KeyCode::Right => app.next_tab(),
					KeyCode::Left => app.previous_tab(),
					KeyCode::Up => app.previous_block(),
					KeyCode::Down => app.next_block(),
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
		.map(|t| Line::from(Span::styled(t, Style::default().fg(Color::Yellow).bold())))
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

fn draw_blocks_tab<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
		.split(area);

	let blocks: Vec<ListItem> = app
		.block_headers
		.items
		.iter()
		.rev()
		.map(|h| {
			ListItem::new(vec![Line::from(Span::styled(
				format!("{:?} > {:?}", h.number(), h.hash()),
				Style::default().fg(Color::Yellow),
			))])
		})
		.collect();

	let l = List::new(blocks)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title(format!("Latest {} Blocks", BLOCKS_MAX_LIMIT)),
		)
		.style(Style::default().fg(Color::Yellow))
		.highlight_style(Style::default().add_modifier(Modifier::BOLD))
		.highlight_symbol("> ");
	f.render_stateful_widget(l, chunks[0], &mut app.block_headers.state);

	let text = vec![Line::from("Block Details Page")];
	let block = Block::default()
		.borders(Borders::ALL)
		.title("Block Details")
		.style(Style::default().fg(Color::Yellow));
	let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
	f.render_widget(paragraph, chunks[1]);
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

pub struct StatefulList<T> {
	pub state: ListState,
	pub items: VecDeque<T>,
}

impl<T> StatefulList<T> {
	pub fn with_items(items: VecDeque<T>) -> StatefulList<T> {
		StatefulList { state: ListState::default(), items }
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) =>
				if i >= self.items.len() - 1 {
					0
				} else {
					i + 1
				},
			None => 0,
		};
		log::debug!(target: "cli", "detect down key, select, {:?}", i);
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) =>
				if i == 0 {
					self.items.len() - 1
				} else {
					i - 1
				},
			None => 0,
		};
		log::debug!(target: "cli", "detect up key, select, {:?}", i);
		self.state.select(Some(i));
	}
}
