use tui::{
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{keys, style::SharedTheme, ui::widgets::DrawableComponent};

pub struct PlayerList {
    pub focus: bool,
    pub selection: usize,
    theme: SharedTheme,
}

impl PlayerList {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            selection: 0,
            theme,
        }
    }
}

impl DrawableComponent for PlayerList {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        // let title = format!("Player [{}]", keys::get_hint(app.key_config.player_list));

        // let block = Block::default()
        //     .title(title)
        //     .borders(Borders::ALL)
        //     .border_style(self.theme.block_style(self.focus));
        // let items: Vec<ListItem> = if app.player_roaster.player_count() > 0 {
        //     app.player_roaster
        //         .player_list
        //         .iter()
        //         .map(|p| ListItem::new(p.name.to_string()))
        //         .collect()
        // } else {
        //     vec![ListItem::new("no players existing")]
        // };

        // let player_list = List::new(items).block(block).highlight_style(
        //     Style::default()
        //         .fg(self.theme.selected_item_fg)
        //         .add_modifier(Modifier::BOLD),
        // );

        // let mut list_state = ListState::default();
        // list_state.select(Some(self.selection));

        // f.render_stateful_widget(player_list, rect, &mut list_state);

        Ok(())
    }
}
