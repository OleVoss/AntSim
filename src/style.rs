use std::rc::Rc;

use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

pub type SharedTheme = Rc<Theme>;

pub struct Theme {
    pub selected_tab: Color,
    pub selected_fg: Color,
    pub selected_item_fg: Color,
    pub disabled_fg: Color,
    pub strength_hightlight: Color,
    pub precision_highlight: Color,
    pub endurance_highlight: Color,
    pub luck_highlight: Color,
    pub speed_color: Color,
    pub glide_color: Color,
    pub turn_color: Color,
    pub fade_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected_tab: Color::Gray,
            selected_fg: Color::Gray,
            selected_item_fg: Color::Red,
            disabled_fg: Color::DarkGray,
            strength_hightlight: Color::Red,
            precision_highlight: Color::Blue,
            endurance_highlight: Color::Green,
            luck_highlight: Color::Yellow,
            speed_color: Color::Red,
            glide_color: Color::Rgb(0, 191, 255),
            turn_color: Color::Blue,
            fade_color: Color::Green,
        }
    }
}

impl Theme {
    pub fn init() -> Self {
        Theme::default()
    }
    pub fn block_style(&self, focus: bool) -> Style {
        if focus {
            Style::default()
        } else {
            Style::default().fg(self.disabled_fg)
        }
    }

    pub fn slider_highlight(&self) -> Color {
        todo!();
    }

    pub fn highlight_block(&self) -> Block {
        // maybe need focus
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
    }

    pub fn item(&self, selected: bool) -> Style {
        if selected {
            Style::default().fg(self.selected_item_fg)
        } else {
            Style::default()
        }
    }

    pub fn slider_block(&self, focus: bool) -> Block {
        todo!();
    }

    //  .:-=+*#%@
    pub fn grey_scale(amount: i32) -> char {
        if amount >= 800 {
            return '#';
        } else if amount >= 600 {
            return '%';
        } else if amount >= 450 {
            return '+';
        } else if amount >= 300 {
            return '=';
        } else if amount >= 150 {
            return '-';
        } else if amount >= 100 {
            return ':';
        } else {
            return '.';
        }
    }
}
