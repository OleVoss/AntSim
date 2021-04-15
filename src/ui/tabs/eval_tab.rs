use crate::{
    app::App,
    style::SharedTheme,
    ui::{components::config_vars::ConfigVars, widgets::DrawableComponent},
};

use anyhow::Result;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::GraphType,
    widgets::{Axis, Block, Borders, Chart, Dataset, Sparkline},
    Frame,
};

pub struct EvalTab {
    visible: bool,
}

impl EvalTab {
    pub fn new(theme: SharedTheme) -> Self {
        Self { visible: false }
    }
}

impl DrawableComponent for EvalTab {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
        app: &App,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(30)].as_ref())
            .split(rect);

        let y_food_bounds: [f64; 2] = [
            0.,
            *app.simulation.history.food_total.iter().max().unwrap_or(&0) as f64,
        ];

        let y_ant_bounds: [f64; 2] = [
            0.,
            *vec![
                app.simulation
                    .history
                    .ants
                    .iter()
                    .map(|av| av[0])
                    .max()
                    .unwrap_or(10),
                app.simulation
                    .history
                    .ants
                    .iter()
                    .max()
                    .map(|av| av[1])
                    .unwrap_or(10),
                app.simulation
                    .history
                    .ants
                    .iter()
                    .max()
                    .map(|av| av[2])
                    .unwrap_or(10),
                app.simulation
                    .history
                    .ants
                    .iter()
                    .max()
                    .map(|av| av[3])
                    .unwrap_or(10),
            ]
            .iter()
            .max()
            .unwrap_or(&10) as f64,
        ];

        // datasets
        let total_food_data: Vec<(f64, f64)> = app
            .simulation
            .history
            .step
            .iter()
            .zip(&app.simulation.history.food_total)
            .map(|(s, f)| (*s as f64, *f as f64))
            .collect();

        let food_step_data: Vec<(f64, f64)> = app
            .simulation
            .history
            .food_step
            .iter()
            .zip(&app.simulation.history.food_step)
            .map(|(s, f)| (*s as f64, *f as f64))
            .collect();

        let seeker_data: Vec<(f64, f64)> = app
            .simulation
            .history
            .step
            .iter()
            .zip(&app.simulation.history.ants)
            .map(|(s, f)| (*s as f64, f[0] as f64))
            .collect();

        let returner_data: Vec<(f64, f64)> = app
            .simulation
            .history
            .step
            .iter()
            .zip(&app.simulation.history.ants)
            .map(|(s, f)| (*s as f64, f[1] as f64))
            .collect();

        let follower_data: Vec<(f64, f64)> = app
            .simulation
            .history
            .step
            .iter()
            .zip(&app.simulation.history.ants)
            .map(|(s, f)| (*s as f64, f[2] as f64))
            .collect();

        let noob_data: Vec<(f64, f64)> = app
            .simulation
            .history
            .step
            .iter()
            .zip(&app.simulation.history.ants)
            .map(|(s, f)| (*s as f64, f[3] as f64))
            .collect();

        let slice_bounds = if food_step_data.len() > 100 {
            food_step_data.len() - 100
        } else {
            0
        };

        let step_bounds: [f64; 2] = if food_step_data.len() > 100 {
            [
                food_step_data.len() as f64 - 100.,
                food_step_data.len() as f64 - 1.,
            ]
        } else {
            [0., app.simulation.history.step.len() as f64]
        };

        let mut food_labals = if y_food_bounds[1] > 25. {
            vec![]
        } else {
            vec![String::from("0"), String::from("25")]
        };

        let mut ant_labels = if y_ant_bounds[1] > 10. {
            vec![]
        } else {
            vec![String::from("0"), String::from("10")]
        };

        let mut step_labels = if step_bounds[1] > 40. {
            vec![]
        } else {
            vec![String::from("0"), String::from("20")]
        };

        for i in (y_food_bounds[0] as i32..y_food_bounds[1] as i32).step_by(25) {
            food_labals.push(i.to_string());
        }

        for i in (y_ant_bounds[0] as i32..y_ant_bounds[1] as i32).step_by(10) {
            ant_labels.push(i.to_string());
        }

        for i in (step_bounds[0] as i32..step_bounds[1] as i32).step_by(20) {
            step_labels.push(i.to_string());
        }

        let food_datasets = vec![
            Dataset::default()
                .name("Total food")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Yellow))
                .data(&total_food_data[slice_bounds..]),
            Dataset::default()
                .name("Step food")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Red))
                .data(&food_step_data[slice_bounds..]),
        ];

        let ant_datasets = vec![
            Dataset::default()
                .name("Seeker")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Blue))
                .data(&seeker_data[slice_bounds..]),
            Dataset::default()
                .name("Returner")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Red))
                .data(&returner_data[slice_bounds..]),
            Dataset::default()
                .name("Follower")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Green))
                .data(&follower_data[slice_bounds..]),
            Dataset::default()
                .name("Noobs")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Yellow))
                .data(&noob_data[slice_bounds..]),
        ];

        let food_chart = Chart::new(food_datasets)
            .block(
                Block::default()
                    .title("Total food over time")
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("steps")
                    .labels(step_labels.iter().cloned().map(Span::from).collect())
                    .bounds(step_bounds)
                    .style(Style::default().fg(Color::White)),
            )
            .y_axis(
                Axis::default()
                    .title("Total food")
                    .labels(food_labals.iter().cloned().map(Span::from).collect())
                    .bounds(y_food_bounds)
                    .style(Style::default().fg(Color::White)),
            );

        let ant_chart = Chart::new(ant_datasets)
            .block(
                Block::default()
                    .title("Total ant states")
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("steps")
                    .labels(step_labels.iter().cloned().map(Span::from).collect())
                    .bounds(step_bounds)
                    .style(Style::default().fg(Color::White)),
            )
            .y_axis(
                Axis::default()
                    .title("Ant state count")
                    .labels(ant_labels.iter().cloned().map(Span::from).collect())
                    .bounds(y_ant_bounds)
                    .style(Style::default().fg(Color::White)),
            );

        f.render_widget(food_chart, chunks[0]);
        f.render_widget(ant_chart, chunks[1]);

        Ok(())
    }
}
