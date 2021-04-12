use std::rc::Rc;

use anyhow::{private::new_adhoc, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use enum_index::{EnumIndex, IndexEnum};

use crate::{
    app::App,
    config::{self, SharedConfig},
    core::simulation::simulation::Simulation,
    keys::{KeyConfig, SharedKeyConfig},
    ui::tabs::{discs::DiscTabSections, PlayerTabSections},
    UI,
};

pub fn key_event(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
        app.should_quit = true;
        return Ok(());
    }

    if ev == ui.key_config.tab_overview
        || ev == ui.key_config.tab_config
        || ev == ui.key_config.tab_simulation
        || ev == ui.key_config.tab_player
        || ev == ui.key_config.tab_discs
    {
        ui.switch_tab(ev)?;
    }

    if ev == app.key_config.select {
        app.load_player();
    }

    match ui.tab {
        0 => (),
        1 => (),
        2 => (),
        3 => player_tab(app, ev, ui)?,
        4 => disc_tab(app, ev, ui)?,
        _ => (),
    }

    Ok(())
}

fn disc_tab(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    Ok(())
}
// ! this is not good
fn player_tab(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    //     if ev == app.key_config.player_list {
    //         ui.player_tab.select(PlayerTabSections::Player);
    //     } else if ev == app.key_config.slider_list {
    //         ui.player_tab.select(PlayerTabSections::Stats);
    //     } else if ev == app.key_config.disc_bag {
    //         ui.player_tab.select(PlayerTabSections::Bag);
    //     }

    //     match ui.player_tab.focused() {
    //         PlayerTabSections::Player => {
    //             if ev == app.key_config.move_down {
    //                 let player_count = app.player_roaster.player_count();
    //                 let new_index =
    //                     (ui.player_tab.player_list.selection + player_count + 1) % player_count;
    //                 ui.player_tab.player_list.selection = new_index;
    //                 ui.player_tab.stats_list.player_index = new_index; // ! this is bad
    //             } else if ev == app.key_config.move_up {
    //                 let player_count = app.player_roaster.player_count();
    //                 let new_index =
    //                     (ui.player_tab.player_list.selection + player_count - 1) % player_count;
    //                 ui.player_tab.player_list.selection = new_index;
    //                 ui.player_tab.stats_list.player_index = new_index; // ! same
    //             }
    //         }
    //         PlayerTabSections::Stats => {
    //             if ev == app.key_config.move_down {
    //                 let stats_count = PlayerStatType::VARIANT_COUNT;
    //                 let new_index =
    //                     (ui.player_tab.stats_list.selection + stats_count + 1) % stats_count;
    //                 ui.player_tab.stats_list.selection = new_index;
    //             } else if ev == app.key_config.move_up {
    //                 let stats_count = PlayerStatType::VARIANT_COUNT;
    //                 let new_index =
    //                     (ui.player_tab.stats_list.selection + stats_count - 1) % stats_count;
    //                 ui.player_tab.stats_list.selection = new_index;
    //             } else if ev == app.key_config.move_right {
    //                 let player_index = ui.player_tab.player_list.selection;
    //                 match app.player_roaster.by_index_mut(player_index) {
    //                     Some(player) => {
    //                         // get stat type
    //                         let stat_type: PlayerStatType =
    //                             PlayerStatType::index_enum(ui.player_tab.stats_list.selection)
    //                                 .unwrap_or(PlayerStatType::Strength);
    //                         // get stat value
    //                         let stat_value = player.stat_value(stat_type);
    //                         // update player stat
    //                         let new_value = stat_value + app.config.stat_increment_step(stat_type);
    //                         player.change_stat_value(new_value, stat_type);
    //                     }
    //                     None => {}
    //                 }
    //             } else if ev == app.key_config.move_left {
    //                 let player_index = ui.player_tab.player_list.selection;
    //                 match app.player_roaster.by_index_mut(player_index) {
    //                     Some(player) => {
    //                         // get stat type
    //                         let stat_type: PlayerStatType =
    //                             PlayerStatType::index_enum(ui.player_tab.stats_list.selection)
    //                                 .unwrap_or(PlayerStatType::Strength);
    //                         // get stat value
    //                         let stat_value = player.stat_value(stat_type);
    //                         // update player stat
    //                         let new_value = stat_value - app.config.stat_increment_step(stat_type);
    //                         player.change_stat_value(new_value, stat_type);
    //                     }
    //                     None => {}
    //                 }
    //             }
    //         }
    //         PlayerTabSections::Bag => {}
    //     }
    Ok(())
}

fn simulation_tab(app: &mut App, ev: KeyEvent, ui: &mut UI) -> Result<()> {
    if ev == app.key_config.start_simulation {
    } else if ev == app.key_config.step_simulation {
    }

    Ok(())
}
