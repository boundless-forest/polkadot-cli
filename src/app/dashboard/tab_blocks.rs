// crates.io
use ratatui::{
	prelude::{
		text::Line, Backend, Color, Constraint, Direction, Frame, Layout, Modifier, Rect, Span,
		Style,
	},
	widgets::*,
};
use sp_core::Encode;
use sp_runtime::{
	traits::{Block as BlockT, Hash, Header as HeaderT},
	DigestItem,
};
// this crate
use super::{DashBoard, BLOCKS_MAX_LIMIT};
use crate::networks::ChainInfo;

pub fn draw_blocks_tab<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>, area: Rect)
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
