use figlet_rs::{FIGfont, FIGure};
use ron::value;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Paragraph, Widget},
};

pub const HIGHT: u16 = 7;
pub struct NumberBox<'a> {
    pub value: f64,
    block: Option<Block<'a>>,
    name: Option<String>,
    color: Color,
}

impl<'a> NumberBox<'a> {
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        return self;
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        return self;
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        return self;
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        return self;
    }
}

impl<'a> Default for NumberBox<'a> {
    fn default() -> Self {
        Self {
            value: 0.0,
            name: Some(String::from("Speed")),
            color: Color::White,
            block: Some(Block::default()),
        }
    }
}

impl<'a> Widget for NumberBox<'a> {
    fn render(mut self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // areas
        let box_area = match self.block.take() {
            Some(mut b) => {
                let inner_area = b.inner(area);
                b = match self.name {
                    Some(n) => {
                        b = b.title(n);
                        b
                    }
                    None => b,
                };
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(23), Constraint::Percentage(30)].as_ref())
            .split(box_area);

        // area: value
        let value_area = Rect::new(box_area.x, box_area.y, box_area.width - 3, box_area.height);

        // rendering

        let box_style = Style::default().fg(self.color);

        let small_font = FIGfont::from_file("assets/rectangles.flf").unwrap();
        let figure = small_font.convert(&self.value.to_string());
        let value_para = match figure {
            Some(f) => figure_to_paragraph(f).alignment(Alignment::Center),
            None => Paragraph::new(Span::raw("content")),
        };
        value_para.style(box_style).render(chunks[0], buf);

        let x_position = chunks[1].x + chunks[1].width / 2 - 1;
        let y_position_1 = chunks[1].y + 1;
        let y_position_2 = chunks[1].y + 3;
        buf.get_mut(x_position, y_position_1)
            .set_char('\u{25b2}')
            .set_fg(self.color);
        buf.get_mut(x_position, y_position_2)
            .set_char('\u{25bc}')
            .set_fg(self.color);
    }
}

fn figure_to_paragraph(figure: FIGure) -> Paragraph {
    let mut spans: Vec<Spans> = vec![];
    for i in 1..figure.height {
        let mut span_chars: Vec<String> = vec![];
        for character in &figure.characters {
            if let Some(line) = character.characters.get(i as usize) {
                span_chars.push(line.to_string());
            }
        }
        let span = Span::raw(span_chars.join(""));
        spans.push(Spans::from(span));
    }
    Paragraph::new(spans)
}
