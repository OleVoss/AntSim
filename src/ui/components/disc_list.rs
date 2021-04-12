use crate::{app::App, keys, style::SharedTheme, ui::widgets::DrawableComponent};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, ListState, Row, Table, TableState},
    Frame,
};

pub struct DiscList {
    pub selection: usize,
    pub focus: bool,
    theme: SharedTheme,
}

impl DiscList {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            selection: 0,
            focus: false,
            theme,
        }
    }
}

impl DrawableComponent for DiscList {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<()> {
        let title = format!("Discs [{}]", keys::get_hint(app.key_config.disc_list));

        Ok(())
    }
}
