mod tab_blocks;
mod tab_events;
mod tab_system;

// std
use std::{collections::VecDeque, io, sync::Arc};
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
use scale_info::{Path, PortableType, Type, TypeDefSequence};
use scale_value::{scale::decode_as_type, Composite, Value, ValueDef};
use sp_runtime::traits::Header as HeaderT;
use sp_storage::StorageData;
use subxt_metadata::Metadata;
use tokio::sync::mpsc::UnboundedReceiver;
// this crate
use crate::{
	networks::ChainInfo,
	rpc::{BlockForChain, ChainApi, HeaderForChain, RpcClient, SystemPaneInfo},
};
use tab_blocks::draw_blocks_tab;
use tab_events::draw_events_tab;
use tab_system::draw_system;

pub(crate) const BLOCKS_MAX_LIMIT: usize = 30;
pub(crate) const EVENTS_MAX_LIMIT: usize = 5;

pub struct DashBoard<CI: ChainInfo> {
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

pub async fn run_dashboard<B, CI>(
	client: Arc<RpcClient<CI>>,
	terminal: &mut Terminal<B>,
	mut app: DashBoard<CI>,
) -> io::Result<()>
where
	B: Backend,
	CI: ChainInfo,
{
	fn vec_event_records_type_id(metadata: &mut Metadata) -> Option<u32> {
		let event_records_type_id = metadata
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

		let ty_mut = metadata.types_mut();
		let vec_event_records_ty = Type::new(
			Path::default(),
			vec![],
			TypeDefSequence::new(event_records_type_id.into()),
			vec![],
		);
		let vec_event_records_type_id = ty_mut.types.len() as u32;
		ty_mut
			.types
			.push(PortableType { id: vec_event_records_type_id, ty: vec_event_records_ty });

		Some(vec_event_records_type_id)
	}

	let vec_event_records_type_id = vec_event_records_type_id(&mut app.metadata).unwrap();
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
			for data in storage_data {
				let value = decode_as_type(
					&mut data.0.as_ref(),
					vec_event_records_type_id,
					app.metadata.types(),
				);

				if let Ok(event_records) = value {
					match event_records.value {
						ValueDef::Composite(event_records) => match event_records {
							Composite::Named(_) => continue,
							Composite::Unnamed(event_records) =>
								for record in event_records {
									match record.value {
										ValueDef::Composite(inner) => match inner {
											Composite::Named(v) => {
												let event_values: Vec<Value<u32>> = v
													.into_iter()
													.filter(|d| d.0 == "event")
													.map(|d| d.1)
													.collect();

												for event in event_values {
													if app.events.items.len()
														== app.events.items.capacity()
													{
														app.events.items.pop_front();
													} else {
														app.events.items.push_back(event);
													}
												}
											},
											Composite::Unnamed(_) => continue,
										},
										_ => continue,
									}
								},
						},
						_ => continue,
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
