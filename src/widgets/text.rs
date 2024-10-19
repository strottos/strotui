use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Text as RatatiText,
    widgets::WidgetRef,
};

#[derive(Debug)]
pub struct Text<'a> {
    inner: RatatiText<'a>,
    selected: bool,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            inner: RatatiText::from(text),
            selected: false,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'a> Deref for Text<'a> {
    type Target = RatatiText<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Text<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl WidgetRef for Text<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        if self.selected {
            self.inner
                .clone()
                .style(Style::default().fg(Color::Black).bg(Color::White))
                .render_ref(area, buf);
        } else {
            self.inner.render_ref(area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_text() {
        let text = Text::new("Hello, world!");
        let rect = Rect::new(0, 0, 13, 1);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_selected_text() {
        let mut text = Text::new("Hello, world!");
        text.set_selected(true);
        let rect = Rect::new(0, 0, 13, 1);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }
}
