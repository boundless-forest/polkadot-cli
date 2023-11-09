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

fn render_pallet_list<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {}
fn render_pallet_info<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {}
