use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::WidgetRef};

#[derive(Debug)]
pub struct Text {
    text: String,
}

impl<'a> Text {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub(crate) fn get_height(&self, width: u16) -> u16 {
        self.get_lines(width).len() as u16
    }

    fn get_lines(&'a self, width: u16) -> Vec<&'a str> {
        tracing::trace!("Getting lines for text: {:?}", self.text);
        tracing::trace!("Width: {:?}", width);

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

            lines.push(&self.text[pos..to]);

            pos = to;

            while pos < self.text.len() && self.text.chars().nth(pos) == Some(' ') {
                pos += 1;
            }
        }

        tracing::trace!("Lines: {:?}", lines);

        lines
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self::new(text.to_string())
    }
}

impl WidgetRef for Text {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        tracing::trace!("Rendering text into area {:?}: {:?}", area, self.text);

        let width = area.right() - area.left();

        let lines = self.get_lines(width);

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
    fn test_long_text() {
        let text = Text::from("String that is longer than the 40 characters of the rectangle.");
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_long_words_text() {
        let text = Text::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let rect = Rect::new(0, 0, 40, 3);
        let mut buffer = Buffer::empty(rect);

        text.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }
}
