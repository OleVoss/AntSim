use core::num;

use ron::Number;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use super::{number_box, NumberBox};

pub struct BoxListState {
    selected: Option<usize>,
}

impl Default for BoxListState {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl BoxListState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }
}

pub struct BoxList<'a> {
    block: Option<Block<'a>>,
    items: Vec<NumberBox<'a>>,
    style: Style,
    highlight_block: Option<Block<'a>>,
}

impl<'a> BoxList<'a> {
    pub fn new<T>(items: T) -> Self
    where
        T: Into<Vec<NumberBox<'a>>>,
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
        return self;
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

impl<'a> StatefulWidget for BoxList<'a> {
    type State = BoxListState;

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

        let mut slider_offset = list_area.y;

        for (i, mut num_box) in self.items.into_iter().enumerate() {
            if can_show(list_area, slider_offset, number_box::HIGHT) {
                let box_area = Rect::new(
                    list_area.x,
                    slider_offset,
                    list_area.width,
                    number_box::HIGHT,
                );
                if i == state.selected().unwrap_or(0) && self.highlight_block.is_some() {
                    num_box = num_box.block(self.highlight_block.take().unwrap());
                }
                num_box.render(box_area, buf);
                slider_offset += number_box::HIGHT;
            }
        }
    }
}

fn can_show(area: Rect, start: u16, needed_space: u16) -> bool {
    start + needed_space <= area.bottom()
}

#[cfg(test)]
mod test {
    use tui::layout::Rect;

    use super::can_show;

    #[test]
    fn can_show_test() {
        let rect = Rect::new(0, 0, 10, 10);
        let not_show = can_show(rect, 8, 8);
        let show = can_show(rect, 0, 5);
        assert!(!not_show);
        assert!(show);
    }
}
