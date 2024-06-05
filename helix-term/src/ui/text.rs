use crate::compositor::{Component, Context};
use tui::buffer::Buffer as Surface;

use helix_view::graphics::Rect;

pub struct Text {
    pub(crate) contents: tui::text::Text<'static>,
}

impl Text {
    pub fn new(contents: String) -> Self {
        Self {
            contents: tui::text::Text::from(contents),
        }
    }
}

impl From<tui::text::Text<'static>> for Text {
    fn from(contents: tui::text::Text<'static>) -> Self {
        Self { contents }
    }
}

impl Component for Text {
    fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        use tui::widgets::{Paragraph, Widget, Wrap};

        let mut par = Paragraph::new(&self.contents).wrap(Wrap { trim: false });
        if let Some(scroll) = cx.scroll {
            par = par.scroll((scroll as u16, 0))
        }

        par.render(area, surface);
    }

    fn required_size(&mut self, _viewport: (u16, u16)) -> Option<(u16, u16)> {
        Some((self.contents.width() as u16, self.contents.height() as u16))
    }
}

pub fn required_size(text: &tui::text::Text, max_text_width: u16) -> (u16, u16) {
    let mut text_width = 0;
    let mut height = 0;
    for content in &text.lines {
        height += 1;
        let content_width = content.width() as u16;
        if content_width > max_text_width {
            text_width = max_text_width;
            height += content_width.checked_div(max_text_width).unwrap_or(0);
        } else if content_width > text_width {
            text_width = content_width;
        }
    }
    (text_width, height)
}
