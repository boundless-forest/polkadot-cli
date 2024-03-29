mod tab_blocks;
mod tab_events;
mod tab_pallets;
mod tab_system;

// std
use std::{collections::VecDeque, io, sync::Arc};
// crates.io
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use ratatui::{
	prelude::{text::Line, Backend, Color, Constraint, Direction, Frame, Layout, Rect, Span, Style, Terminal},
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

pub(crate) const BLOCKS_MAX_LIMIT: usize = 30;
pub(crate) const EVENTS_MAX_LIMIT: usize = 5;

pub struct DashBoard<'a, CI: ChainInfo> {
	pub metadata: &'a mut Metadata,
	pub system_pane_info: SystemPaneInfo,
	pub blocks_rev: UnboundedReceiver<HeaderForChain<CI>>,
	pub blocks: StatefulList<BlockForChain<CI>>,
	pub selected_block: Option<BlockForChain<CI>>,
	pub events_rev: UnboundedReceiver<Vec<StorageData>>,
	pub events: StatefulList<Value<u32>>,
	pub pallets: StatefulList<(u8, String)>,
	pub selected_pallet: Option<(u8, String)>,
	pub tab_titles: Vec<String>,
	pub selected_tab: usize,
	pub pallet_info_titles: Vec<String>,
	pub selected_pallet_info_tab: usize,
}

impl<'a, CI: ChainInfo> DashBoard<'a, CI> {
	pub(crate) fn new(
		system_pane_info: SystemPaneInfo,
		blocks_rev: UnboundedReceiver<HeaderForChain<CI>>,
		events_rev: UnboundedReceiver<Vec<StorageData>>,
		metadata: &'a mut Metadata,
	) -> DashBoard<'a, CI> {
		let pallets: VecDeque<(u8, String)> = metadata.pallets().map(|p| (p.index(), p.name().to_string())).collect();
		DashBoard {
			metadata,
			system_pane_info,
			blocks_rev,
			events_rev,
			selected_block: None,
			blocks: StatefulList::with_items(VecDeque::with_capacity(BLOCKS_MAX_LIMIT)),
			events: StatefulList::with_items(VecDeque::with_capacity(EVENTS_MAX_LIMIT)),
			pallets: StatefulList::with_items(pallets),
			selected_pallet: None,
			// tab_titles: vec![String::from("Blocks"), String::from("Events"), String::from("Pallets")],
			tab_titles: vec![String::from("Blocks"), String::from("Pallets")],
			selected_tab: 0,
			pallet_info_titles: vec![
				String::from("Constants"),
				String::from("Events"),
				String::from("Errors"),
				String::from("Storages"),
				String::from("Calls"),
			],
			selected_pallet_info_tab: 0,
		}
	}

	async fn subscribe_blocks(&mut self, client: Arc<RpcClient<CI>>) {
		if let Ok(header) = self.blocks_rev.try_recv() {
			if self.blocks.items.len() == self.blocks.items.capacity() {
				self.blocks.items.pop_front();
			}
			if let Ok(signed_block) = client.get_block(header.hash().into()).await {
				self.blocks.items.push_back(signed_block.block);
			}
		}
	}

	async fn _subscribe_events(&mut self) {
		fn vec_event_records_type_id(metadata: &mut Metadata) -> Option<u32> {
			let event_records_type_id = metadata
				.types()
				.types
				.iter()
				.find(|ty| ty.ty.path == Path::from_segments_unchecked(vec!["frame_system".to_string(), "EventRecord".to_string()]))
				.map(|ty| ty.id)
				.unwrap();

			let ty_mut = metadata.types_mut();
			let vec_event_records_ty = Type::new(Path::default(), vec![], TypeDefSequence::new(event_records_type_id.into()), vec![]);
			let vec_event_records_type_id = ty_mut.types.len() as u32;
			ty_mut
				.types
				.push(PortableType { id: vec_event_records_type_id, ty: vec_event_records_ty });

			Some(vec_event_records_type_id)
		}
		let vec_event_records_type_id = vec_event_records_type_id(self.metadata).unwrap();

		if let Ok(storage_data) = self.events_rev.try_recv() {
			for data in storage_data {
				let value = decode_as_type(&mut data.0.as_ref(), vec_event_records_type_id, self.metadata.types());

				if let Ok(event_records) = value {
					match event_records.value {
						ValueDef::Composite(event_records) => match event_records {
							Composite::Named(_) => continue,
							Composite::Unnamed(event_records) =>
								for record in event_records {
									match record.value {
										ValueDef::Composite(inner) => match inner {
											Composite::Named(v) => {
												let event_values: Vec<Value<u32>> =
													v.into_iter().filter(|d| d.0 == "event").map(|d| d.1).collect();

												for event in event_values {
													if self.events.items.len() == self.events.items.capacity() {
														self.events.items.pop_front();
													} else {
														self.events.items.push_back(event);
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
	}
}

pub async fn run_dashboard<'a, B, CI>(
	client: Arc<RpcClient<CI>>,
	terminal: &mut Terminal<B>,
	mut dash_board: DashBoard<'a, CI>,
) -> io::Result<()>
where
	B: Backend,
	CI: ChainInfo,
{
	loop {
		terminal.draw(|f| ui(f, &mut dash_board))?;

		dash_board.subscribe_blocks(client.clone()).await;
		// dash_board.subscribe_events().await;

		if let Event::Key(key) = read()? {
			if key.kind == KeyEventKind::Press {
				match key.code {
					KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
					KeyCode::Tab => {
						dash_board.selected_tab = (dash_board.selected_tab + 1) % dash_board.tab_titles.len();
					},
					KeyCode::Up => match dash_board.selected_tab {
						0 => {
							dash_board.blocks.previous();
							if let Some(i) = dash_board.blocks.state.selected() {
								dash_board.selected_block = dash_board.blocks.items.get(i).cloned();
							}
						},
						// 2 => {
						1 => {
							dash_board.pallets.previous();
							if let Some(i) = dash_board.pallets.state.selected() {
								dash_board.selected_pallet = dash_board.pallets.items.get(i).cloned();
							}
						},
						_ => {},
					},
					KeyCode::Down => match dash_board.selected_tab {
						0 => {
							dash_board.blocks.next();
							if let Some(i) = dash_board.blocks.state.selected() {
								dash_board.selected_block = dash_board.blocks.items.get(i).cloned();
							}
						},
						// 2 => {
						1 => {
							dash_board.pallets.next();
							if let Some(i) = dash_board.pallets.state.selected() {
								dash_board.selected_pallet = dash_board.pallets.items.get(i).cloned();
							}
						},
						_ => {},
					},
					KeyCode::Right =>
					// if dash_board.selected_tab == 2 {
						if dash_board.selected_tab == 1 {
							dash_board.selected_pallet_info_tab =
								(dash_board.selected_pallet_info_tab + 1) % dash_board.pallet_info_titles.len();
						},
					KeyCode::Left =>
					// if dash_board.selected_tab == 2 && dash_board.selected_pallet_info_tab > 0 {
						if dash_board.selected_tab == 1 && dash_board.selected_pallet_info_tab > 0 {
							dash_board.selected_pallet_info_tab -= 1;
						},
					_ => {},
				}
			}
		}
	}
}

fn ui<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>) {
	let size = f.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
		.split(size);

	tab_system::draw_system(f, dash_board, chunks[0]);
	draw_tabs(f, dash_board, chunks[1]);
}

fn draw_tabs<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.split(area);
	let titles = dash_board
		.tab_titles
		.iter()
		.map(|t| Line::from(Span::styled(t, Style::default().fg(Color::Yellow).bold())))
		.collect();
	let tabs = Tabs::new(titles)
		.block(
			Block::default()
				.title(" Chain Information ")
				.title_style(Style::default().bold().italic())
				.border_type(BorderType::Double)
				.borders(Borders::ALL),
		)
		.select(dash_board.selected_tab)
		.style(Style::default().fg(Color::Yellow))
		.highlight_style(Style::default().fg(Color::Cyan));
	f.render_widget(tabs, chunks[0]);

	match dash_board.selected_tab {
		0 => tab_blocks::draw_blocks_tab(f, dash_board, chunks[1]),
		// 1 => tab_events::draw_events_tab(f, dash_board, chunks[1]),
		1 => tab_pallets::draw_pallets_tab(f, dash_board, chunks[1]),
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
