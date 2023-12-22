// crates.io
use ratatui::{
	prelude::{text::Line, Color, Constraint, Direction, Frame, Layout, Modifier, Rect, Span, Style},
	style::Stylize,
	widgets::*,
};
// this crate
use super::DashBoard;
use crate::{handler::print_storage_type, networks::ChainInfo};

pub fn draw_pallets_tab<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
		.split(area);

	render_pallet_list(f, app, chunks[0]);
	render_pallet_info(f, app, chunks[1]);
}

fn render_pallet_list<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {
	let pallets: Vec<ListItem> = app
		.pallets
		.items
		.iter()
		.map(|b| ListItem::new(vec![Line::from(Span::styled(format!("{} > {}", b.0, b.1), Style::default().fg(Color::Yellow)))]))
		.collect();

	let l = List::new(pallets)
		.block(
			Block::default()
				.title_style(Style::default().bold().italic())
				.borders(Borders::ALL)
				.border_type(BorderType::Double)
				.padding(Padding::horizontal(2)),
		)
		.style(Style::default().fg(Color::Yellow))
		.highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
		.highlight_symbol("> ");
	f.render_stateful_widget(l, area, &mut app.pallets.state);
}
fn render_pallet_info<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.split(area);
	let titles = dash_board
		.pallet_info_titles
		.iter()
		.map(|t| Line::from(Span::styled(t, Style::default().fg(Color::Yellow).bold())))
		.collect();
	let tabs = Tabs::new(titles)
		.block(
			Block::default()
				.title(" Pallet Information ")
				.title_style(Style::default().bold().italic())
				.border_type(BorderType::Double)
				.borders(Borders::ALL),
		)
		.select(dash_board.selected_pallet_info_tab)
		.style(Style::default().fg(Color::Yellow))
		.highlight_style(Style::default().fg(Color::Cyan));
	f.render_widget(tabs, chunks[0]);

	match dash_board.selected_pallet_info_tab {
		0 => render_pallet_constants_page(f, dash_board, chunks[1]),
		1 => render_pallet_events_page(f, dash_board, chunks[1]),
		2 => render_pallet_errors_page(f, dash_board, chunks[1]),
		3 => render_pallet_storages_page(f, dash_board, chunks[1]),
		4 => render_pallet_calls_page(f, dash_board, chunks[1]),
		_ => {},
	};
}
fn render_pallet_constants_page<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let block_style = Block::default()
		.title_style(Style::default().bold().italic())
		.borders(Borders::ALL)
		.border_type(BorderType::Double)
		.padding(Padding::horizontal(2))
		.style(Style::default().fg(Color::Yellow));

	if let Some((id, name)) = dash_board.selected_pallet.clone() {
		let pallet = dash_board.metadata.pallets().find(|p| p.name() == name && p.index() == id);
		if let Some(pallet) = pallet {
			let constants: Vec<ListItem> = pallet
				.constants()
				.into_iter()
				.map(|c| {
					let ty_id = c.ty();
					let mut bytes = c.value();
					let value = scale_value::scale::decode_as_type(&mut bytes, ty_id, dash_board.metadata.types()).unwrap();
					ListItem::new(vec![Line::from(Span::styled(format!("> {} : {}", c.name(), value), Style::default().fg(Color::Yellow)))])
				})
				.collect();

			if constants.len() != 0 {
				let l = List::new(constants).block(block_style);
				f.render_widget(l, area);
			} else {
				let text = "None".to_string();
				let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
				f.render_widget(paragraph, area);
			}
		}
	}
}
fn render_pallet_events_page<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let block_style = Block::default()
		.title_style(Style::default().bold().italic())
		.borders(Borders::ALL)
		.border_type(BorderType::Double)
		.style(Style::default().fg(Color::Yellow));

	let text = "Events Page".to_string();
	let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
fn render_pallet_errors_page<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let block_style = Block::default()
		.title_style(Style::default().bold().italic())
		.borders(Borders::ALL)
		.border_type(BorderType::Double)
		.style(Style::default().fg(Color::Yellow));

	let text = "Errors Page".to_string();
	let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
fn render_pallet_storages_page<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let block_style = Block::default()
		.title_style(Style::default().bold().italic())
		.borders(Borders::ALL)
		.border_type(BorderType::Double)
		.padding(Padding::horizontal(2))
		.style(Style::default().fg(Color::Yellow));

	if let Some((id, name)) = dash_board.selected_pallet.clone() {
		let pallet = dash_board.metadata.pallets().find(|p| p.name() == name && p.index() == id);
		if let Some(pallet) = pallet {
			if let Some(s) = pallet.storage() {
				let storages: Vec<ListItem> = s
					.entries()
					.into_iter()
					.map(|e| {
						ListItem::new(vec![Line::from(Span::styled(
							format!("> {} : {}", e.name(), &print_storage_type(e.entry_type().clone(), &dash_board.metadata)),
							Style::default().fg(Color::Yellow),
						))])
					})
					.collect();

				if storages.len() != 0 {
					let l = List::new(storages).block(block_style);
					f.render_widget(l, area);
				} else {
					let text = "None".to_string();
					let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
					f.render_widget(paragraph, area);
				}
			} else {
				let text = "None".to_string();
				let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
				f.render_widget(paragraph, area);
			}
		}
	}
}
fn render_pallet_calls_page<CI: ChainInfo>(f: &mut Frame, dash_board: &mut DashBoard<CI>, area: Rect) {
	let block_style = Block::default()
		.title_style(Style::default().bold().italic())
		.borders(Borders::ALL)
		.border_type(BorderType::Double)
		.style(Style::default().fg(Color::Yellow));

	let text = "Calls Page".to_string();
	let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
