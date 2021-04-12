use tui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

use crate::{style::SharedTheme, ui::widgets::DrawableComponent};

pub struct Scorecard {
    theme: SharedTheme,
}

impl Scorecard {
    pub fn new(theme: SharedTheme) -> Self {
        Self { theme }
    }
}

impl DrawableComponent for Scorecard {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
        app: &crate::app::App,
    ) -> anyhow::Result<()> {
        // let block = Block::default().title("Scorecard").borders(Borders::ALL);
        // let rows: Vec<Row> = Vec::new();
        // for p in &app.player_roaster.player_list {
        //     let mut row_vec: Vec<Cell> = Vec::new();
        //     for i in 1..app.simulation.course_length() {
        //         let score = app.simulation.get_player_round_score(p, i);
        //         let score_str = String::from(score.strokes.len().to_string());
        //         row_vec.push(Cell::from(score_str));
        //     }
        // }
        // let mut widths: Vec<Constraint> = Vec::new();
        // widths.push(Constraint::Percentage(15));
        // for i in 0..8 {
        //     widths.push(Constraint::Length(4));
        // }
        // let table = Table::new(rows)
        //     .header(
        //         Row::new(vec!["Name", "1", "2", "3", "4"])
        //             .style(Style::default().add_modifier(Modifier::UNDERLINED)),
        //     )
        //     .widths(&widths)
        //     .block(block)
        //     .column_spacing(1);

        // let mut table_state = TableState::default();
        // f.render_stateful_widget(table, rect, &mut table_state);

        Ok(())
    }
}
