use crate::{
    app::App,
    style::SharedTheme,
    ui::{components::config_vars::ConfigVars, widgets::DrawableComponent},
};
use anyhow::Result;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct ConfigTab {
    visible: bool,
    pub parameter: ConfigVars,
}

impl ConfigTab {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            visible: false,
            parameter: ConfigVars::new(theme.clone()),
        }
    }
}

impl DrawableComponent for ConfigTab {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(rect);

        self.parameter.draw(f, chunks[0], app)?;

        Ok(())
    }
}
