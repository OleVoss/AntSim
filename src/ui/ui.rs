use std::rc::Rc;

use crate::{
    app::App,
    config::{Config, SharedConfig},
    keys::{KeyConfig, SharedKeyConfig},
    style::{SharedTheme, Theme},
};
use anyhow::{bail, Result};
use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use super::{
    tabs::{ConfigTab, EvalTab, Simulation},
    widgets::DrawableComponent,
};

pub struct UI {
    theme: SharedTheme,
    pub key_config: SharedKeyConfig,
    pub config: SharedConfig,
    pub tab: usize,
    pub config_tab: ConfigTab,
    pub simulation_tab: Simulation,
    pub eval_tab: EvalTab,
}

impl UI {
    pub fn new() -> Self {
        let theme = Rc::new(Theme::init());
        Self {
            theme: theme.clone(),
            key_config: Rc::new(KeyConfig::init()),
            config: Rc::new(Config::init()),
            tab: 0,
            config_tab: ConfigTab::new(theme.clone()),
            simulation_tab: Simulation::new(theme.clone()),
            eval_tab: EvalTab::new(theme),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, app: &App) -> Result<()> {
        let fsize = f.size();

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(fsize);

        self.draw_tabs(f, chunks_main[0], app);

        // TODO tab selection in UI rather than in APP
        match self.tab {
            0 => self.simulation_tab.draw(f, chunks_main[1], app)?,
            1 => self.config_tab.draw(f, chunks_main[1], app)?,
            2 => self.eval_tab.draw(f, chunks_main[1], app)?,
            _ => bail!("unknown tab"),
        };

        Ok(())
    }

    fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, r: Rect, app: &App) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        // TODO: https://github.com/extrawurst/gitui/blob/master/src/app.rs Zeile 641-strings editable with config usw.
        let tabs = [
            Span::raw("Simulation [1]"),
            Span::raw("Config [2]"),
            Span::raw("Evaluation [3]"),
        ]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .border_style(Style::default()),
                )
                .style(Style::default().add_modifier(Modifier::DIM))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::UNDERLINED)
                        .remove_modifier(Modifier::DIM),
                )
                .divider("|")
                .select(self.tab),
            r,
        );
    }
}

impl UI {
    pub fn switch_tab(&mut self, k: KeyEvent) -> Result<()> {
        if k == self.key_config.tab_simulation {
            self.set_tab(0)?;
        } else if k == self.key_config.tab_config {
            self.set_tab(1)?;
        } else if k == self.key_config.tab_eval {
            self.set_tab(2)?;
        }

        Ok(())
    }

    pub fn set_tab(&mut self, tab: usize) -> Result<()> {
        self.tab = tab;
        Ok(())
    }
}
