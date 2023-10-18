// std
use std::{collections::VecDeque, io, sync::Arc};
// crates.io
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use frame_system::EventRecord;
use ratatui::{
	prelude::{
		text::Line, Backend, Color, Constraint, Direction, Frame, Layout, Modifier, Rect, Span,
		Style, Terminal,
	},
	style::Stylize,
	widgets::*,
};
use scale_info::{
	form::{MetaForm, PortableForm},
	Path, TypeDefSequence,
};
use scale_value::{Composite, Value, ValueDef};
use sp_core::Encode;
use sp_runtime::{
	traits::{Block as BlockT, Hash, HashFor, Header as HeaderT},
	DigestItem,
};
use sp_storage::StorageData;
use subxt_metadata::Metadata;
use tokio::sync::mpsc::UnboundedReceiver;
// this crate
use crate::{
	networks::ChainInfo,
	rpc::{BlockForChain, ChainApi, HeaderForChain, RpcClient, SystemPaneInfo},
};

const BLOCKS_MAX_LIMIT: usize = 30;
const EVENTS_MAX_LIMIT: usize = 5;

pub(crate) struct DashBoard<CI: ChainInfo> {
	pub metadata: Metadata,
	pub system_pane_info: SystemPaneInfo,
	pub blocks_rev: UnboundedReceiver<HeaderForChain<CI>>,
	pub blocks: StatefulList<BlockForChain<CI>>,
	pub selected_block: Option<BlockForChain<CI>>,
	pub events_rev: UnboundedReceiver<Vec<StorageData>>,
	pub events: StatefulList<Value<u32>>,
	pub tab_titles: Vec<String>,
	pub index: usize,
}

impl<CI: ChainInfo> DashBoard<CI> {
	pub(crate) fn new(
		system_pane_info: SystemPaneInfo,
		blocks_rev: UnboundedReceiver<HeaderForChain<CI>>,
		events_rev: UnboundedReceiver<Vec<StorageData>>,
		metadata: Metadata,
	) -> DashBoard<CI> {
		DashBoard {
			metadata,
			system_pane_info,
			blocks_rev,
			events_rev,
			selected_block: None,
			blocks: StatefulList::with_items(VecDeque::with_capacity(BLOCKS_MAX_LIMIT)),
			events: StatefulList::with_items(VecDeque::with_capacity(EVENTS_MAX_LIMIT)),
			tab_titles: vec![String::from("Blocks"), String::from("Events")],
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
		self.blocks.previous();
		if let Some(i) = self.blocks.state.selected() {
			self.selected_block = self.blocks.items.get(i).cloned();
		}
	}

	pub fn next_block(&mut self) {
		self.blocks.next();
		if let Some(i) = self.blocks.state.selected() {
			self.selected_block = self.blocks.items.get(i).cloned();
		}
	}
}

pub(crate) async fn run_dashboard<B, CI>(
	client: Arc<RpcClient<CI>>,
	terminal: &mut Terminal<B>,
	mut app: DashBoard<CI>,
) -> io::Result<()>
where
	B: Backend,
	CI: ChainInfo,
{
	fn find_event_record_type_id(metadata: &mut Metadata) -> Option<u32> {
		let event_record_type_id = metadata
			.types()
			.types
			.iter()
			.find(|ty| {
				ty.ty.path
					== Path::from_segments_unchecked(vec![
						"frame_system".to_string(),
						"EventRecord".to_string(),
					])
			})
			.map(|ty| ty.id)
			.unwrap();

		use scale_info::Type;

		let ty_mut = metadata.types_mut();
		let vec_events_ty = Type::new(
			Path::default(),
			vec![],
			TypeDefSequence::new(event_record_type_id.into()),
			vec![],
		);
		let vec_events_type_id = ty_mut.types.len() as u32;
		ty_mut
			.types
			.push(scale_info::PortableType { id: vec_events_type_id, ty: vec_events_ty });

		Some(vec_events_type_id)
	}

	let vec_events_type_id = find_event_record_type_id(&mut app.metadata).unwrap();
	loop {
		terminal.draw(|f| ui(f, &mut app))?;

		if let Ok(header) = app.blocks_rev.try_recv() {
			if app.blocks.items.len() == app.blocks.items.capacity() {
				app.blocks.items.pop_front();
			}
			if let Ok(signed_block) = client.get_block(header.hash().into()).await {
				app.blocks.items.push_back(signed_block.block);
			}
		}

		if let Ok(storage_data) = app.events_rev.try_recv() {
			log::debug!(target: "cli", "storage_data length receive : {:?}", storage_data.len());
			for data in storage_data {
				// TODO: Handle the Error
				let value = scale_value::scale::decode_as_type(
					&mut data.0.as_ref(),
					vec_events_type_id,
					app.metadata.types(),
				);

				if let Ok(value) = value {
					match value.value {
						ValueDef::Composite(composite) => match composite {
							Composite::Named(_) => continue,
							Composite::Unnamed(vec_outer) =>
								for value in vec_outer {
									match value.value {
										ValueDef::Composite(composite_inner) =>
											match composite_inner {
												Composite::Named(named_data) => {
													let value: Vec<Value<u32>> = named_data
														.into_iter()
														.filter(|d| d.0 == "event")
														.map(|d| d.1)
														.collect();

													log::debug!(target: "cli", "have extracted events {:?}", value);
													for v in value {
														if app.events.items.len()
															== app.events.items.capacity()
														{
															app.events.items.pop_front();
														} else {
															app.events.items.push_back(v);
														}
													}
												},
												Composite::Unnamed(_) => continue,
											},
										_ => continue,
									}
								},
						},
						_ => {
							log::debug!(target: "cli", "diverse branch 1");
							continue;
						},
					}
				}
			}
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
		1 => draw_events_tab(f, app, chunks[1]),
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
		.constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
		.split(area);

	let blocks: Vec<ListItem> = app
		.blocks
		.items
		.iter()
		.map(|b| {
			ListItem::new(vec![Line::from(Span::styled(
				format!("{:?} > {:?}", b.header().number(), b.header().hash()),
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
		.highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
		.highlight_symbol("> ");
	f.render_stateful_widget(l, chunks[0], &mut app.blocks.state);

	let block = Block::default()
		.borders(Borders::ALL)
		.title("Block Details")
		.style(Style::default().fg(Color::Yellow));
	let selected_block = app.selected_block.clone().or(app.blocks.items.back().cloned());
	if let Some(b) = selected_block {
		// Fixed items
		let mut items = vec![
			ListItem::new(format!("ParentHash     => {:?}", b.header().parent_hash())),
			ListItem::new(format!("Number         => {:?}", b.header().number())),
			ListItem::new(format!("StateRoot      => {:?}", b.header().state_root())),
			ListItem::new(format!("ExtrinsicRoot  => {:?}", b.header().extrinsics_root())),
		];

		// Logs
		items.push(ListItem::new("Digest         => ".to_string()));
		for (index, item) in b.header().digest().logs().iter().enumerate() {
			let message = match item {
				DigestItem::PreRuntime(id, data) => {
					let id = String::from_utf8_lossy(id);
					let data = array_bytes::bytes2hex("0x", data);
					format!("PreRuntime[{}]: {}", id, data)
				},
				DigestItem::Consensus(id, data) => {
					let id = String::from_utf8_lossy(id);
					let data = array_bytes::bytes2hex("0x", data);
					format!("Consensus[{}]: {}", id, data)
				},
				DigestItem::Seal(id, data) => {
					let id = String::from_utf8_lossy(id);
					let data = array_bytes::bytes2hex("0x", data);
					format!("Seal[{}]: {}", id, data)
				},
				DigestItem::Other(data) => {
					let data = array_bytes::bytes2hex("0x", data);
					format!("Other: {}", data)
				},
				DigestItem::RuntimeEnvironmentUpdated => "RuntimeEnvironmentUpdated".to_string(),
			};

			items.push(ListItem::new(format!("          log{index} => {}", message)));
		}

		// Extrinsics
		items.push(ListItem::new("Extrinsic      => ".to_string()));
		for (i, e) in b.extrinsics().iter().enumerate() {
			items.push(ListItem::new(format!(
				"          ext{i} => {:?}",
				CI::Hashing::hash(&e.encode())
			)));
		}

		let l = List::new(items).block(block);
		f.render_widget(l, chunks[1]);
	} else {
		let text = "Block Details Page".to_string();
		let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
		f.render_widget(paragraph, chunks[1]);
	}
}

fn draw_events_tab<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>, area: Rect)
where
	B: Backend,
	CI: ChainInfo,
{
	let mut text = "".to_string();
	for e in &app.events.items {
		text.push_str(&format!(
			"{}\n",
			serde_json::to_string(e).unwrap_or("Decode Error Occurred.".to_string())
		));
	}
	let l = Paragraph::new(text)
		.wrap(Wrap { trim: true })
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title(format!("Latest {} Events", EVENTS_MAX_LIMIT)),
		)
		.style(Style::default().fg(Color::Yellow));
	f.render_widget(l, area);
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
		self.state.select(Some(i));
	}
}
