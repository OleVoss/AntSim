use std::{borrow::BorrowMut, rc::Rc};

use anyhow::{private::new_adhoc, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use enum_index::{EnumIndex, IndexEnum};

use crate::{
    app::App,
    config::{self, SharedConfig},
    core::simulation::simulation::Simulation,
    keys::{KeyConfig, SharedKeyConfig},
    UI,
};

pub fn key_event(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
        app.should_quit = true;
        return Ok(());
    }

    if ev == ui.key_config.tab_config
        || ev == ui.key_config.tab_simulation
        || ev == ui.key_config.tab_eval
    {
        ui.switch_tab(ev)?;
    }

    match ui.tab {
        0 => simulation_tab(app, ev, ui)?,
        1 => config_tab(app, ev, ui)?,
        _ => (),
    }

    Ok(())
}

fn config_tab(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == app.key_config.move_down {
        let param_count = app.config.vars().len();
        let new_index = (ui.config_tab.parameter.selection + param_count + 1) % param_count;
        ui.config_tab.parameter.selection = new_index;
    } else if ev == app.key_config.move_up {
        let param_count = app.config.vars().len();
        let new_index = (ui.config_tab.parameter.selection + param_count - 1) % param_count;
        ui.config_tab.parameter.selection = new_index;
    } else if ev == app.key_config.move_right {
        let mut vars_mut = app.config.vars_mut();
        if let Some(var) = vars_mut.get_mut(ui.config_tab.parameter.selection) {
            var.incr();
        }
    } else if ev == app.key_config.move_left {
        let mut vars_mut = app.config.vars_mut();
        if let Some(var) = vars_mut.get_mut(ui.config_tab.parameter.selection) {
            var.decr();
        }
    }
    Ok(())
}

fn simulation_tab(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == app.key_config.start_simulation {
    } else if ev == app.key_config.step_simulation {
    } else if ev == app.key_config.spawn_ant {
        app.simulation.spwan_ant();
    } else if ev == app.key_config.span_ant_bulk {
        for _ in 0..10 {
            app.simulation.spwan_ant();
        }
    } else if ev == app.key_config.reset_sim {
        app.simulation.reset(app.config);
    } else if ev == app.key_config.pause_sim {
        app.simulation.paused = !app.simulation.paused;
    }

    Ok(())
}
