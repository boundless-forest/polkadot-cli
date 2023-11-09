// crates.io
use ratatui::{
	prelude::{text::Line, Color, Constraint, Direction, Frame, Layout, Modifier, Rect, Span, Style},
	style::Stylize,
	widgets::*,
};
// this crate
use super::DashBoard;
use crate::networks::ChainInfo;

pub fn draw_pallets_tab<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
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
				.title(format!(" Pallets "))
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
fn render_pallet_info<CI: ChainInfo>(f: &mut Frame, _app: &mut DashBoard<CI>, area: Rect) {
	let block_style = Block::default()
		.title(" Pallet Details ")
		.title_style(Style::default().bold().italic())
		.borders(Borders::ALL)
		.border_type(BorderType::Double)
		.style(Style::default().fg(Color::Yellow));

	let text = "Pallets Details Page".to_string();
	let paragraph = Paragraph::new(text).block(block_style).wrap(Wrap { trim: true });
	f.render_widget(paragraph, area);
}
