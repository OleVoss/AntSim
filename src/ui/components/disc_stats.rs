use tui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, BorderType, Borders},
};

use crate::{
    style::SharedTheme,
    ui::widgets::{BoxList, BoxListState, DrawableComponent, NumberBox},
};

pub struct DiscStats {
    pub focus: bool,
    pub selection: usize,
    pub disc_index: usize,
    theme: SharedTheme,
}

impl DiscStats {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            focus: false,
            selection: 0,
            disc_index: 0,
            theme,
        }
    }
}

impl DrawableComponent for DiscStats {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        // stat boxes
        let mut boxes = Vec::<NumberBox>::new();

        // for stat in AVAILABLE_DISC_STATS.iter() {
        //     let stat_value = match disc {
        //         Some(d) => d.stat_value(*stat),
        //         None => 0.0,
        //     };

        //     let stat_bounds: DiscStatBounds = match app.config.get_disc_bounds(*stat) {
        //         Some(b) => *b,
        //         None => {
        //             let bounds = DiscStatBounds::default();
        //             bounds
        //         }
        //     };

        //     let num_box = NumberBox::default()
        //         .value(stat_value)
        //         .block(Block::default().borders(Borders::ALL))
        //         .color(self.theme.box_color(*stat))
        //         .name(stat.to_string());
        //     boxes.push(num_box);
        // }

        // let box_list = BoxList::new(boxes)
        //     .block(Block::default().borders(Borders::ALL).title("Stats"))
        //     .highlight_block(
        //         Block::default()
        //             .borders(Borders::ALL)
        //             .border_type(BorderType::Double),
        //     );
        // let mut state = BoxListState::default();
        // state.select(Some(self.selection));

        // f.render_stateful_widget(box_list, rect, &mut state);
        Ok(())
    }
}
