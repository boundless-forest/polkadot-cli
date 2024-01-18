// crates.io
use ratatui::{
	prelude::{Color, Frame, Rect, Style},
	style::Stylize,
	widgets::*,
};
// this crate
use super::{DashBoard, EVENTS_MAX_LIMIT};
use crate::networks::ChainInfo;

pub fn _draw_events_tab<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {
	let mut text = "".to_string();
	for e in &app.events.items {
		text.push_str(&format!("{}\n", serde_json::to_string(e).unwrap_or("Decode Error Occurred.".to_string())));
	}

	let l = Paragraph::new(text)
		.wrap(Wrap { trim: true })
		.block(
			Block::default()
				.title(format!(" Latest {} Events ", EVENTS_MAX_LIMIT))
				.title_style(Style::default().bold().italic())
				.border_type(BorderType::Double)
				.borders(Borders::ALL),
		)
		.style(Style::default().fg(Color::Yellow));
	f.render_widget(l, area);
}
