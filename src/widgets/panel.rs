//! Code for a strotui Panel.
//!
//! A Panel is a container that can hold other widgets. It can be scrolled and you can optionally
//! select things within it.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Padding, Scrollbar, ScrollbarState, StatefulWidget, WidgetRef},
};

use super::Text;

#[derive(Debug)]
pub enum PanelWidget {
    Text(Text),
}

impl PanelWidget {
    fn get_height(&self, width: u16) -> u16 {
        match self {
            PanelWidget::Text(text) => text.get_height(width),
        }
    }
}

#[derive(Debug, Default)]
pub struct PanelBuilder {
    pub title: Option<String>,
    pub borders: Option<Borders>,
    pub padding: Option<Padding>,
    pub scrollbar: bool,
    pub children: Vec<PanelWidget>,
}

impl PanelBuilder {
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn add_child(mut self, widget: PanelWidget) -> Self {
        self.children.push(widget);
        self
    }

    pub fn build<'a>(self) -> Panel<'a> {
        Panel {
            block: self.title.map(|t| {
                Block::default()
                    .title(t)
                    .borders(self.borders.unwrap_or(Borders::ALL))
                    .padding(self.padding.unwrap_or(Padding::symmetric(2, 1)))
            }),
            scrollbar: self.scrollbar,
            children: self.children,
        }
    }
}

#[derive(Debug)]
pub struct Panel<'a> {
    block: Option<Block<'a>>,
    scrollbar: bool,
    // TODO: style: Style,
    children: Vec<PanelWidget>,
}

impl<'a> Panel<'a> {
    pub fn new_builder(title: Option<String>) -> PanelBuilder {
        PanelBuilder {
            title,
            borders: None,
            padding: None,
            scrollbar: true,
            children: Vec::new(),
        }
    }

    pub fn add_text(&'a mut self, text: Text) {
        self.children.push(PanelWidget::Text(text));
    }

    fn render_outer(&self, area: Rect, buf: &mut Buffer) -> Rect {
        if let Some(block) = self.block.as_ref() {
            block.render_ref(area, buf);
            block.inner(area)
        } else {
            area
        }
    }

    fn render_children(&self, area: Rect, buf: &mut Buffer) -> usize {
        let mut y = area.top() as usize;
        let width = area.right() - area.left();
        tracing::trace!("area {:?}", area);

        for child in &self.children {
            let height = child.get_height(width).min(area.bottom() - y as u16);

            if y <= area.bottom() as usize {
                let child_area = Rect::new(area.left(), y as u16, width, height);
                tracing::trace!("Rendering child in area {:?}", child_area);
                match child {
                    PanelWidget::Text(text) => text.render_ref(child_area, buf),
                }
            }
            y += height as usize;
        }

        y
    }

    fn render_scrollbar(
        &self,
        area: Rect,
        buf: &mut Buffer,
        children_height: usize,
        display_height: usize,
    ) {
        if self.scrollbar {
            let scrollbar_area = Rect::new(
                area.left(),
                area.top() + 1,
                area.right() - area.left(),
                area.bottom() - area.top() - 2,
            );
            let scrollbar_height = children_height.saturating_sub(display_height);
            let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));
            let mut scrollbar_state = ScrollbarState::new(scrollbar_height).position(0);
            scrollbar.render(scrollbar_area, buf, &mut scrollbar_state);
        }
    }
}

impl WidgetRef for Panel<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let inner = self.render_outer(area, buf);

        let children_height = self.render_children(inner, buf);

        self.render_scrollbar(
            area,
            buf,
            children_height,
            (inner.bottom() - inner.top()) as usize,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_simple_text_lines() {
        let panel = Panel::new_builder(Some("Panel Test".to_string()))
            .padding(Padding::symmetric(0, 0))
            .add_child(PanelWidget::Text(Text::from("Hello 1!")))
            .add_child(PanelWidget::Text(Text::from("Hello 2!")))
            .add_child(PanelWidget::Text(Text::from("Hello 3!")))
            .build();

        let rect = Rect::new(0, 0, 40, 12);
        let mut buffer = Buffer::empty(rect);

        panel.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }

    #[traced_test]
    #[test]
    fn test_simple_long_text_lines() {
        let panel = Panel::new_builder(Some("Panel Test".to_string()))
            .add_child(PanelWidget::Text(Text::from("Let's make several strings that are longer than the 40 characters of the rectangle.")))
            .add_child(PanelWidget::Text(Text::from("Let's make several strings that are longer than the 40 characters of the rectangle.")))
            .add_child(PanelWidget::Text(Text::from("Let's make several strings that are longer than the 40 characters of the rectangle.")))
            .build();

        let rect = Rect::new(0, 0, 40, 12);
        let mut buffer = Buffer::empty(rect);

        panel.render_ref(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }
}
