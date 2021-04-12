use tui::widgets::{Block, Borders};

use crate::{style::SharedTheme, ui::widgets::DrawableComponent};

pub struct Flightpath {
    pub focus: bool,
    theme: SharedTheme,
}

impl Flightpath {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            theme,
        }
    }
}

impl DrawableComponent for Flightpath {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let title = "Flightpath";
        let block = Block::default().title(title).borders(Borders::ALL);

        f.render_widget(block, rect);
        Ok(())
    }
}
