use std::borrow::Borrow;

use tui::{
    style::Style,
    widgets::{Block, Borders},
};

use crate::{
    keys,
    style::SharedTheme,
    ui::widgets::{DrawableComponent, Slider, SliderList, SliderListState},
};

pub struct PlayerStats {
    pub selection: usize,
    pub player_index: usize,
    pub focus: bool,
    theme: SharedTheme,
}

impl PlayerStats {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            selection: 0,
            player_index: 0,
            focus: false,
            theme,
        }
    }
}

impl DrawableComponent for PlayerStats {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        // let player = app.player_roaster.by_index(self.player_index);
        // let mut slider_list: Vec<Slider> = Vec::<Slider>::new();
        // for stat in AVAILABLE_PLAYER_STATS.iter() {
        //     let stat_value = match player {
        //         Some(p) => p.stat_value(*stat),
        //         None => 0,
        //     };

        //     let stat_bounds: StatBounds = match app.config.get_player_bounds(*stat) {
        //         Some(b) => *b,
        //         None => {
        //             let bounds = StatBounds::default();
        //             bounds
        //         }
        //     };

        //     let slider = Slider::default()
        //         .ignore_bounds(false)
        //         .from(stat_bounds.from() as f64)
        //         .to(stat_bounds.to() as f64)
        //         .value(stat_value as f64) // has to be used after from/to
        //         .highlight_style(Style::default().fg(self.theme.slider_highlight(*stat)))
        //         .label(stat.to_string())
        //         .block(
        //             Block::default()
        //                 .border_style(self.theme.block_style(self.focus))
        //                 .borders(Borders::ALL),
        //         );

        //     slider_list.push(slider);
        // }

        // let title = format!("Stats [{}]", keys::get_hint(app.key_config.slider_list));

        // let slider_list = SliderList::new(slider_list)
        //     .block(
        //         Block::default()
        //             .title(title)
        //             .borders(Borders::ALL)
        //             .border_style(self.theme.block_style(self.focus)),
        //     )
        //     .highlight_block(self.theme.highlight_block())
        //     .style(self.theme.block_style(self.focus));
        // let mut state = SliderListState::default();
        // state.select(Some(self.selection));

        // f.render_stateful_widget(slider_list, rect, &mut state);
        Ok(())
    }
}
