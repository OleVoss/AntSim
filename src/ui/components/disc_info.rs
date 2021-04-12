use std::convert::TryInto;

use tui::{
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{Block, Borders},
};

use crate::{keys, style::SharedTheme, ui::widgets::DrawableComponent};

use super::{disc_stats::DiscStats, flightpath::Flightpath};

pub struct DiscInfo {
    pub focus: bool,
    theme: SharedTheme,
    pub stats: DiscStats,
    flightpath: Flightpath,
}

impl DiscInfo {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            theme: theme.clone(),
            stats: DiscStats::new(theme.clone()),
            flightpath: Flightpath::new(theme),
        }
    }
}

impl DrawableComponent for DiscInfo {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let title = format!("Disc Info [{}]", keys::get_hint(app.key_config.disc_info));
        let block = Block::default().title(title).borders(Borders::ALL);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)].as_ref())
            .split(rect);

        // f.render_widget(block, rect);
        self.stats.draw(f, chunks[0], app)?;
        self.flightpath.draw(f, chunks[1], app)?;
        Ok(())
    }
}
