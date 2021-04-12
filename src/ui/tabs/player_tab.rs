use crate::{
    app::App,
    keys,
    style::SharedTheme,
    ui::{
        components::{disc_bag::DiscBag, player_list::PlayerList, player_stats_list},
        widgets::{DrawableComponent, Slider, SliderList, SliderListState},
    },
};

use player_stats_list::PlayerStats;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTabSections {
    Player,
    Stats,
    Bag,
}
pub struct PlayerTab {
    visible: bool,
    focus: PlayerTabSections,
    pub player_list: PlayerList,
    pub stats_list: PlayerStats,
    pub disc_bag: DiscBag,
}

impl PlayerTab {
    pub fn new(theme: SharedTheme) -> Self {
        let mut tab = Self {
            visible: false,
            focus: PlayerTabSections::Player,
            player_list: PlayerList::new(theme.clone()),
            stats_list: PlayerStats::new(theme.clone()),
            disc_bag: DiscBag::new(theme),
        };
        tab.player_list.focus = true;
        return tab;
    }

    pub fn focused(&self) -> PlayerTabSections {
        self.focus
    }

    pub fn select(&mut self, section: PlayerTabSections) {
        self.focus = section;
        match self.focus {
            PlayerTabSections::Player => {
                self.player_list.focus = true;
                self.stats_list.focus = false;
                self.disc_bag.focus = false;
            }
            PlayerTabSections::Stats => {
                self.player_list.focus = false;
                self.stats_list.focus = true;
                self.disc_bag.focus = false;
            }
            PlayerTabSections::Bag => {
                self.player_list.focus = false;
                self.stats_list.focus = false;
                self.disc_bag.focus = true;
            }
        }
    }
}

impl DrawableComponent for PlayerTab {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &App,
    ) -> anyhow::Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(rect);

        // player section
        self.player_list.draw(f, main_chunks[0], &app)?;
        // stats section
        self.stats_list.draw(f, main_chunks[1], &app)?;
        // bag section
        self.disc_bag.draw(f, main_chunks[2], &app)?;

        Ok(())
    }
}
