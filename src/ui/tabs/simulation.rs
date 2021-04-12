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
    pub scorecard: Scorecard,
}

impl Simulation {
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            visible: false,
            scorecard: Scorecard::new(theme),
        }
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
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(80),
                    Constraint::Percentage(13),
                ]
                .as_ref(),
            )
            .split(rect);

        // TODO: call the draw() function of the specific components
        let scorecard_block = Block::default()
            .title("Scorecard")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let hole_view = Block::default()
            .title("Hole")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain);

        let renderer = PrintRenderer::new(' ', 't', 'B', 'w', 'x', 'o');

        let final_string =
            renderer.render(main_chunks[1].width as i64, main_chunks[1].height as i64);

        let mut text: Vec<Spans> = Vec::new();
        for s in final_string {
            text.push(Spans::from(vec![Span::raw(s.to_owned())]))
        }
        let para = Paragraph::new(text)
            .block(hole_view)
            // .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        f.render_widget(para, main_chunks[1]);
        self.scorecard.draw(f, main_chunks[2], app)?;

        Ok(())
    }
}
