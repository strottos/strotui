//! A simple text widget that displays text that can over multiple lines or truncated to fit the
//! width of the widget.

use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::WidgetRef};

#[derive(Debug)]
pub enum TextWrap {
    Truncate,
    TruncateWithEllipsis,
    Wrapped,
    WrappedWords,
    WrappedJustified,
    WrappedCentered,
    WrappedRightAligned,
}

/// A simple text widget that displays text that can be either
/// (a) Truncated to fit the width of the widget, or
/// (b) Truncated to fit the width of the widget with an ellipsis,
/// (c) Wrapped to fit the width of the widget exactly,
/// (d) Wrapped to fit the width of the widget at word boundaries (default),
/// (e) Wrapped and justified to fit the width of the widget as word boundaries
/// (f) Wrapped and centered to fit the width of the widget as word boundaries, or
/// (g) Wrapped and right-aligned to fit the width of the widget as word boundaries.
#[derive(Debug)]
pub struct Text {
    text: String,
    wrap: TextWrap,
    // TODO: line_numbers: bool,
}

impl<'a> Text {
    pub fn new(text: String) -> Self {
        Self {
            text,
            wrap: TextWrap::WrappedWords,
        }
    }

    pub fn new_with_wrap(text: String, wrap: TextWrap) -> Self {
        Self { text, wrap }
    }

    pub(crate) fn get_height(&self, width: u16) -> u16 {
        self.get_lines(width).len() as u16
    }

    // Possibly consider memoizing this, we effectively do the same work twice for panels, though
    // it should be pretty fast I think. Maybe we should profile it at some point for different
    // wrappings.
    fn get_lines(&'a self, width: u16) -> Vec<&'a str> {
        tracing::trace!("Getting lines at width {} for text: {:?}", width, self);

        match self.wrap {
            TextWrap::Truncate => self.get_lines_truncate(width),
            TextWrap::TruncateWithEllipsis => self.get_lines_truncate(width), // Ellipsis handlded by the renderer
            TextWrap::Wrapped => self.get_lines_wrapped(width),
            TextWrap::WrappedWords => self.get_lines_wrapped_words(width),
            TextWrap::WrappedJustified => self.get_lines_wrapped_justified(width),
            TextWrap::WrappedCentered => self.get_lines_wrapped_centered(width),
            TextWrap::WrappedRightAligned => self.get_lines_wrapped_right_aligned(width),
        }
    }

    fn get_lines_truncate(&'a self, width: u16) -> Vec<&'a str> {
        let end = self.text.len().min(width as usize);
        vec![&self.text[..end]]
    }

    fn get_lines_wrapped(&'a self, width: u16) -> Vec<&'a str> {
        let mut pos = 0;
        let mut lines = vec![];

        while pos <= self.text.len() {
            let end = pos + width as usize;
            let end = end.min(self.text.len());
            let line = &self.text[pos..end];
            if line.is_empty() {
                break;
            }

            if let Some(to) = line.find('\n') {
                lines.push(&line[..to]);
                pos += to + 1;
                continue;
            }

            lines.push(line);

            pos = end;
        }

        tracing::trace!("Lines: {:?}", lines);

        lines
    }

    fn get_lines_wrapped_words(&'a self, width: u16) -> Vec<&'a str> {
        let mut pos = 0;
        let mut lines = vec![];

        while pos <= self.text.len() {
            let end = pos + width as usize;
            let end = end.min(self.text.len());
            let line = &self.text[pos..end];
            if line.is_empty() {
                break;
            }

            let to = if self.text.len() == end || self.text[end..].starts_with(' ') {
                end
            } else {
                line.rfind(' ').map(|y| pos + y).unwrap_or(end)
            };

            if let Some(to) = line.find('\n') {
                lines.push(&line[..to]);
                pos += to + 1;
                continue;
            }

            lines.push(&self.text[pos..to]);

            pos = to;

            while pos < self.text.len() && self.text.chars().nth(pos) == Some(' ') {
                pos += 1;
            }
        }

        tracing::trace!("Lines: {:?}", lines);

        lines
    }

    fn get_lines_wrapped_justified(&'a self, width: u16) -> Vec<&'a str> {
        todo!();
    }

    fn get_lines_wrapped_centered(&'a self, width: u16) -> Vec<&'a str> {
        todo!();
    }

    fn get_lines_wrapped_right_aligned(&'a self, width: u16) -> Vec<&'a str> {
        todo!();
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self::new(text.to_string())
    }
}

impl WidgetRef for Text {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        tracing::trace!("Rendering text into area {:?}: {:?}", area, self);

        let width = area.right() - area.left();

        let lines = self.get_lines(width);

        if let TextWrap::TruncateWithEllipsis = self.wrap {
            debug_assert!(lines.len() == 1);
            if lines[0].len() == width as usize {
                buf.set_stringn(
                    area.left(),
                    area.top(),
                    &lines[0][..(width as usize - 3)],
                    width.into(),
                    Style::default(),
                );
                buf.set_stringn(
                    area.left() + width - 3,
                    area.top(),
                    "...",
                    3,
                    Style::default(),
                );
                return;
            }
        }

        lines
            .iter()
            .enumerate()
            .filter(|(y, _)| y + (area.top() as usize) < (area.bottom() as usize))
            .for_each(|(y, line)| {
                tracing::trace!(
                    "Rendering line at {} {}: {:?}",
                    area.left(),
                    area.top() + y as u16,
                    line
                );
                buf.set_stringn(
                    area.left(),
                    area.top() + y as u16,
                    line,
                    width.into(),
                    Style::default(),
                );
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_text() {
        let text = Text::from("Hello, world!");
        let rect = Rect::new(0, 0, 13, 1);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    // TODO: Styles

    #[traced_test]
    #[test]
    fn test_words_with_truncate() {
        let text = Text::new_with_wrap(
            "Let's not wrap this text even though it's plenty long enough to do so.".to_string(),
            TextWrap::Truncate,
        );
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_words_with_truncate_and_ellipsis() {
        let text = Text::new_with_wrap(
            "Let's not wrap this text even though it's plenty long enough to do so.".to_string(),
            TextWrap::TruncateWithEllipsis,
        );
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_with_wrap() {
        let text = Text::new_with_wrap(
            "Let's wrap this text that is long enough to do so.".to_string(),
            TextWrap::Wrapped,
        );
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_with_wrap_and_newlines() {
        let text = Text::new_with_wrap(
            "Let's wrap this text that is long enough to do so.\nAnd it has a newline.".to_string(),
            TextWrap::Wrapped,
        );
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_long_text_word_wrap() {
        let text = Text::from("String that is longer than the 40 characters of the rectangle.");
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_long_words_text_word_wrap() {
        let text = Text::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_with_word_wrap_and_newlines() {
        let text =
            Text::from("Let's wrap this text that is long enough to do so.\nAnd it has a newline.");
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    // TODO: Test with newlines and wrapping
}
