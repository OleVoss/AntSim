use crate::{
    app::App,
    style::SharedTheme,
    ui::{
        components::{disc_info::DiscInfo, disc_list::DiscList},
        widgets::DrawableComponent,
    },
};
use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{self, Constraint, Direction, Layout, Rect},
    Frame,
};

#[derive(Debug, Clone, Copy)]
pub enum DiscTabSections {
    Discs,
    Info,
}

pub struct DiscTab {
    visible: bool,
    focus: DiscTabSections,
    pub disc_list: DiscList,
    pub disc_info: DiscInfo,
}

impl DiscTab {
    pub fn new(theme: SharedTheme) -> Self {
        let mut tab = Self {
            visible: false,
            focus: DiscTabSections::Discs,
            disc_list: DiscList::new(theme.clone()),
            disc_info: DiscInfo::new(theme),
        };
        tab.select(DiscTabSections::Discs);
        return tab;
    }

    pub fn focused(&self) -> DiscTabSections {
        self.focus
    }

    pub fn select(&mut self, section: DiscTabSections) {
        self.focus = section;
        match self.focus {
            DiscTabSections::Discs => {
                self.disc_list.focus = true;
                self.disc_info.focus = false;
            }
            DiscTabSections::Info => {
                self.disc_list.focus = false;
                self.disc_info.focus = true;
            }
        }
    }
}

impl DrawableComponent for DiscTab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect, app: &App) -> Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(rect);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Min(1)].as_ref())
            .split(main_chunks[1]);

        self.disc_list.draw(f, main_chunks[0], app)?;
        self.disc_info.draw(f, main_chunks[1], app)?;
        Ok(())
    }
}
