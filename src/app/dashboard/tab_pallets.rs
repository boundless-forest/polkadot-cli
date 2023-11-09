// crates.io
use ratatui::{
	prelude::{Color, Frame, Rect, Style},
	widgets::*,
};
// this crate
use super::{DashBoard, EVENTS_MAX_LIMIT};
use crate::networks::ChainInfo;

pub fn draw_pallets_tab<CI: ChainInfo>(f: &mut Frame, app: &mut DashBoard<CI>, area: Rect) {}
