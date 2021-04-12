use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::rc::Rc;

pub type SharedKeyConfig = Rc<KeyConfig>;

#[derive(Debug)]
pub struct KeyConfig {
    pub tab_overview: KeyEvent,
    pub tab_simulation: KeyEvent,
    pub start_simulation: KeyEvent,
    pub step_simulation: KeyEvent,
    pub tab_config: KeyEvent,
    pub tab_player: KeyEvent,
    pub player_list: KeyEvent,
    pub slider_list: KeyEvent,
    pub disc_bag: KeyEvent,
    pub tab_discs: KeyEvent,
    pub disc_list: KeyEvent,
    pub disc_info: KeyEvent,
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
            tab_simulation: KeyEvent {
                code: KeyCode::Char('2'),
                modifiers: KeyModifiers::empty(),
            },
            start_simulation: KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::SHIFT,
            },
            step_simulation: KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::empty(),
            },
            tab_config: KeyEvent {
                code: KeyCode::Char('3'),
                modifiers: KeyModifiers::empty(),
            },
            // player tab
            tab_player: KeyEvent {
                code: KeyCode::Char('4'),
                modifiers: KeyModifiers::empty(),
            },
            player_list: KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::empty(),
            },
            slider_list: KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::empty(),
            },
            disc_bag: KeyEvent {
                code: KeyCode::Char('b'),
                modifiers: KeyModifiers::empty(),
            },
            // Disc Tab
            tab_discs: KeyEvent {
                code: KeyCode::Char('5'),
                modifiers: KeyModifiers::empty(),
            },
            disc_list: KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::empty(),
            },
            disc_info: KeyEvent {
                code: KeyCode::Char('i'),
                modifiers: KeyModifiers::empty(),
            },
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
