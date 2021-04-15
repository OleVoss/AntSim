use std::borrow::Borrow;

use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    config::parameter_desc,
    keys,
    style::SharedTheme,
    ui::widgets::{DrawableComponent, Slider, SliderList, SliderListState},
};

pub struct ConfigVars {
    pub selection: usize,
    pub focus: bool,
    theme: SharedTheme,
}

impl ConfigVars {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            selection: 0,
            focus: true,
            theme,
        }
    }
}

impl DrawableComponent for ConfigVars {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(60), Constraint::Min(50)].as_ref())
            .split(rect);

        let mut slider_list: Vec<Slider> = Vec::<Slider>::new();
        for config_var in app.config.vars() {
            let slider = Slider::default()
                .ignore_bounds(false)
                .from(config_var.min() as f64)
                .to(config_var.max() as f64)
                .value(config_var.val() as f64) // has to be used after from/to
                .highlight_style(Style::default().fg(Color::Blue))
                .label(config_var.name().to_string())
                .block(
                    Block::default()
                        .border_style(self.theme.block_style(self.focus))
                        .borders(Borders::ALL),
                );

            slider_list.push(slider);
        }

        let title = format!(
            "Parameter [{}]",
            keys::get_hint(app.key_config.parameter_slider)
        );

        let slider_list = SliderList::new(slider_list)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(self.theme.block_style(self.focus)),
            )
            .highlight_block(self.theme.highlight_block())
            .style(self.theme.block_style(self.focus));
        let mut state = SliderListState::default();
        state.select(Some(self.selection));

        let text = vec![
            Spans::from(vec![Span::styled(
                parameter_desc[self.selection][0],
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::raw(parameter_desc[self.selection][1])]),
        ];

        let para = Paragraph::new(text)
            .block(Block::default().title("Description").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        f.render_stateful_widget(slider_list, main_chunks[0], &mut state);
        f.render_widget(para, main_chunks[1]);
        Ok(())
    }
}
