use itertools::Itertools;
use ratatui::{style::Style, text::Span};

#[derive(Debug, Default)]
pub struct HighlightedLine<'a> {
    text: &'a str,
    text_style: Style,
    high_style: Style,
    highlight: Vec<usize>,
}

impl<'a> HighlightedLine<'a> {
    pub fn text(self, text: &'a str) -> Self {
        Self { text, ..self }
    }
    pub fn text_style(self, text_style: Style) -> Self {
        Self { text_style, ..self }
    }
    pub fn highlight_style(self, high_style: Style) -> Self {
        Self { high_style, ..self }
    }
    pub fn highlights(self, higlights: impl IntoIterator<Item = usize>) -> Self {
        Self {
            highlight: higlights.into_iter().collect(),
            ..self
        }
    }
}

impl<'a> From<HighlightedLine<'a>> for ratatui::text::Line<'a> {
    fn from(value: HighlightedLine<'a>) -> Self {
        let mut spans = value
            .text
            .char_indices()
            .map(|(i, c)| (i, i + c.len_utf8()))
            .map(|(start, end)| &value.text[start..end])
            .map(|slice| (slice, value.text_style))
            .collect_vec();
        for i in value.highlight {
            let (slice, _) = spans[i];
            spans[i] = (slice, value.high_style)
        }

        spans
            .into_iter()
            .map(|(content, style)| Span::styled(content, style))
            .collect_vec()
            .into()
    }
}

impl<'a> From<HighlightedLine<'a>> for ratatui::text::Text<'a> {
    fn from(value: HighlightedLine<'a>) -> Self {
        let line: ratatui::text::Line<'a> = value.into();
        line.into()
    }
}
