use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::{keys, style::SharedTheme, ui::widgets::DrawableComponent};

pub struct DiscBag {
    pub focus: bool,
    selection: usize,
    theme: SharedTheme,
}

impl DiscBag {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            selection: 0,
            theme,
        }
    }
}

impl DrawableComponent for DiscBag {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let bag_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(rect);

        let title = format!("Bag [{}]", keys::get_hint(app.key_config.disc_bag));

        let bag_block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(self.theme.block_style(self.focus))
            .border_type(BorderType::Plain);

        let path_block = Block::default()
            .title("Path")
            .borders(Borders::ALL)
            .border_style(self.theme.block_style(self.focus))
            .border_type(BorderType::Plain);

        f.render_widget(bag_block, bag_chunks[1]);
        f.render_widget(path_block, bag_chunks[0]);
        Ok(())
    }
}
