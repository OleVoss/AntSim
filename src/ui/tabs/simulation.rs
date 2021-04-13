use anyhow::Result;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::App,
    main,
    style::SharedTheme,
    ui::{components::scorecard::Scorecard, widgets::DrawableComponent},
    utils::renderer::{PrintRenderer, Renderer},
};

pub struct Simulation {
    pub visible: bool,
}

impl Simulation {
    pub fn new(theme: SharedTheme) -> Self {
        Self { visible: false }
    }
}

impl DrawableComponent for Simulation {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App,
    ) -> Result<()> {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(rect);

        let hole_view = Block::default()
            .title("Map")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let renderer = PrintRenderer::new('H', 'F', 'O', '*');

        let width = main_chunks[0].width as i32;
        let height = main_chunks[0].height as i32;
        let final_string = renderer.render(
            &app.simulation.map,
            app.simulation.colony.ants.clone(),
            rect.width.into(),
            rect.height.into(),
        );

        let mut text: Vec<Spans> = Vec::new();
        for s in final_string {
            text.push(Spans::from(vec![Span::raw(s.to_owned())]))
        }
        let para = Paragraph::new(text)
            .block(hole_view)
            // .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White));

        f.render_widget(para, rect);

        Ok(())
    }
}
