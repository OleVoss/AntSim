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
    tabs::{DiscTab, Overview, PlayerTab, Simulation},
    widgets::DrawableComponent,
};

pub struct UI {
    theme: SharedTheme,
    pub key_config: SharedKeyConfig,
    pub config: SharedConfig,
    pub tab: usize,
    overview_tab: Overview,
    simulation_tab: Simulation,
    pub player_tab: PlayerTab,
    pub disc_tab: DiscTab,
}

impl UI {
    pub fn new() -> Self {
        let theme = Rc::new(Theme::init());
        Self {
            theme: theme.clone(),
            key_config: Rc::new(KeyConfig::init()),
            config: Rc::new(Config::init()),
            tab: 1,
            overview_tab: Overview::new(),
            simulation_tab: Simulation::new(theme.clone()),
            player_tab: PlayerTab::new(theme.clone()),
            disc_tab: DiscTab::new(theme),
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
            0 => self.overview_tab.draw(f, chunks_main[1], app)?,
            1 => self.simulation_tab.draw(f, chunks_main[1], app)?,
            3 => self.player_tab.draw(f, chunks_main[1], app)?,
            4 => self.disc_tab.draw(f, chunks_main[1], app)?,
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
            Span::raw("Overview [1]"),
            Span::raw("Simulation [2]"),
            Span::raw("Config [3]"),
            Span::raw("Player [4]"),
            Span::raw("Discs [5]"),
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
        if k == self.key_config.tab_overview {
            self.set_tab(0)?;
        } else if k == self.key_config.tab_simulation {
            self.set_tab(1)?;
        } else if k == self.key_config.tab_config {
            self.set_tab(2)?;
        } else if k == self.key_config.tab_player {
            self.set_tab(3)?;
        } else if k == self.key_config.tab_discs {
            self.set_tab(4)?;
        }

        Ok(())
    }

    pub fn set_tab(&mut self, tab: usize) -> Result<()> {
        self.tab = tab;
        Ok(())
    }
}
