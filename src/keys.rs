use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::rc::Rc;

pub type SharedKeyConfig = Rc<KeyConfig>;

#[derive(Debug)]
pub struct KeyConfig {
    pub tab_overview: KeyEvent,
    // simulation
    pub tab_simulation: KeyEvent,
    pub start_simulation: KeyEvent,
    pub step_simulation: KeyEvent,
    pub spawn_ant: KeyEvent,
    pub span_ant_bulk: KeyEvent,
    pub reset_sim: KeyEvent,
    pub pause_sim: KeyEvent,

    // config
    pub tab_config: KeyEvent,
    pub parameter_slider: KeyEvent,

    // evaluation
    pub tab_eval: KeyEvent,

    // general
    pub select: KeyEvent,
    pub move_up: KeyEvent,
    pub move_down: KeyEvent,
    pub move_left: KeyEvent,
    pub move_right: KeyEvent,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            tab_overview: KeyEvent {
                code: KeyCode::Char('1'),
                modifiers: KeyModifiers::empty(),
            },

            // simulation
            tab_simulation: KeyEvent {
                code: KeyCode::Char('1'),
                modifiers: KeyModifiers::empty(),
            },
            start_simulation: KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            },
            step_simulation: KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::empty(),
            },
            spawn_ant: KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::empty(),
            },
            span_ant_bulk: KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            },
            reset_sim: KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::CONTROL,
            },
            pause_sim: KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::empty(),
            },

            // config
            tab_config: KeyEvent {
                code: KeyCode::Char('2'),
                modifiers: KeyModifiers::empty(),
            },
            parameter_slider: KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::empty(),
            },
            tab_eval: KeyEvent {
                code: KeyCode::Char('3'),
                modifiers: KeyModifiers::empty(),
            },

            // general
            select: KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::empty(),
            },
            move_up: KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::empty(),
            },
            move_down: KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::empty(),
            },
            move_left: KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::empty(),
            },
            move_right: KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::empty(),
            },
        }
    }
}

impl KeyConfig {
    pub fn init() -> Self {
        Self::default()
    }
}

pub fn get_hint(ev: KeyEvent) -> String {
    match ev.code {
        KeyCode::Char(c) => format!("{}{}", get_modifier_hint(ev.modifiers), c),
        _ => format!(""),
    }
}

fn get_modifier_hint(modifier: KeyModifiers) -> String {
    match modifier {
        KeyModifiers::CONTROL => "^".to_string(),
        KeyModifiers::SHIFT => {
            "\u{21e7}".to_string() //
        }
        _ => "".to_string(),
    }
}
