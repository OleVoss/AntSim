use std::default;

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, StatefulWidget, Widget},
};

use super::Slider;

#[derive(Debug, Clone)]
pub struct SliderListState {
    selected: Option<usize>,
}

impl Default for SliderListState {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl SliderListState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

pub struct SliderList<'a> {
    block: Option<Block<'a>>,
    items: Vec<Slider<'a>>,
    style: Style,
    highlight_block: Option<Block<'a>>,
}

impl<'a> SliderList<'a> {
    pub fn new<T>(items: T) -> Self
    where
        T: Into<Vec<Slider<'a>>>,
    {
        Self {
            block: None,
            style: Style::default(),
            items: items.into(),
            highlight_block: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn highlight_block(mut self, block: Block<'a>) -> Self {
        self.highlight_block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> StatefulWidget for SliderList<'a> {
    type State = SliderListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let mut list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for (i, mut slider) in self.items.into_iter().enumerate() {
            if list_area.y + Slider::HIGHT <= area.height {
                if i == state.selected().unwrap_or(0) && self.highlight_block.is_some() {
                    slider = slider.block(self.highlight_block.clone().unwrap());
                    // ok because is_some is checked
                }
                let slider_area =
                    Rect::new(list_area.x, list_area.y, list_area.width, Slider::HIGHT);
                slider.render(slider_area, buf);
                list_area.y += Slider::HIGHT;
            }
        }
    }
}
