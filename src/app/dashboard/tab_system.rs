// crates.io
use ratatui::{
	prelude::{Backend, Color, Constraint, Direction, Frame, Layout, Rect, Style},
	style::Stylize,
	widgets::*,
};
// this crate
use super::DashBoard;
use crate::networks::ChainInfo;

pub fn draw_system<B, CI>(f: &mut Frame<B>, app: &mut DashBoard<CI>, area: Rect)
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
