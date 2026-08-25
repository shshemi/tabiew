use std::borrow::Cow;

use ratatui::{
    layout::Alignment,
    style::Modifier,
    text::{Line, Span},
};

use crate::misc::config::theme;

#[derive(Debug, Default)]
enum TagLineStyle {
    #[default]
    MultiColor,
    MonoColor,
}

#[derive(Debug, Default)]
pub struct TagLine<'a> {
    tags: Vec<Tag<'a>>,
    style: TagLineStyle,
    alignment: Alignment,
}

impl<'a> TagLine<'a> {
    pub fn multi_color() -> Self {
        Self {
            tags: Default::default(),
            style: TagLineStyle::MultiColor,
            alignment: Default::default(),
        }
    }

    pub fn mono_color() -> Self {
        Self {
            tags: Default::default(),
            style: TagLineStyle::MonoColor,
            alignment: Default::default(),
        }
    }

    pub fn tag(mut self, tag: Tag<'a>) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn centered(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn right_aligned(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }

    pub fn left_aligned(mut self) -> Self {
        self.alignment = Alignment::Left;
        self
    }
}

impl From<TagLine<'_>> for Line<'_> {
    fn from(value: TagLine) -> Self {
        itertools::intersperse(
            value
                .tags
                .into_iter()
                .enumerate()
                .map(|(idx, tag)| match value.style {
                    TagLineStyle::MultiColor => tag.into_multi_color_span(idx).into_iter(),
                    TagLineStyle::MonoColor => tag.into_mono_color_span().into_iter(),
                }),
            [Span::raw(" "), Span::raw("")].into_iter(),
        )
        .flatten()
        .collect::<Line<'_>>()
        .alignment(value.alignment)
    }
}

#[derive(Debug)]
pub struct Tag<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> Tag<'a> {
    pub fn new(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    fn into_multi_color_span(self, pos: usize) -> [Span<'static>; 2] {
        [
            Span::raw(format!(" {} ", self.key)).style(theme().tag(pos)),
            Span::raw(format!(" {} ", self.value))
                .style(theme().tag(pos).add_modifier(Modifier::REVERSED)),
        ]
    }

    fn into_mono_color_span(self) -> [Span<'static>; 2] {
        [
            Span::raw(format!(" {} ", self.key)).style(theme().block_tag()),
            Span::raw(format!(" {} ", self.value))
                .style(theme().block_tag().add_modifier(Modifier::REVERSED)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use ratatui::style::Style;

    fn spans(tag_line: TagLine<'_>) -> Vec<(String, Style)> {
        Line::from(tag_line)
            .spans
            .iter()
            .map(|span| (span.content.to_string(), span.style))
            .collect()
    }

    #[test]
    fn no_tags_produce_no_spans() {
        assert!(spans(TagLine::multi_color()).is_empty());
    }

    #[test]
    fn a_tag_becomes_a_padded_key_and_value_span() {
        let cells = spans(TagLine::multi_color().tag(Tag::new("k", "v")));

        assert_eq!(
            cells.iter().map(|(text, _)| text.as_str()).collect_vec(),
            vec![" k ", " v "]
        );
    }

    #[test]
    fn the_value_span_is_reversed_against_the_key() {
        let cells = spans(TagLine::multi_color().tag(Tag::new("k", "v")));

        assert_eq!(cells[0].1, theme().tag(0));
        assert_eq!(cells[1].1, theme().tag(0).add_modifier(Modifier::REVERSED));
    }

    #[test]
    fn tags_are_separated_from_each_other() {
        let cells = spans(
            TagLine::multi_color()
                .tag(Tag::new("a", "1"))
                .tag(Tag::new("b", "2")),
        );

        assert_eq!(
            cells.iter().map(|(text, _)| text.as_str()).collect_vec(),
            vec![" a ", " 1 ", " ", "", " b ", " 2 "]
        );
    }

    #[test]
    fn multi_color_walks_the_theme_palette_per_tag() {
        let cells = spans(
            TagLine::multi_color()
                .tag(Tag::new("a", "1"))
                .tag(Tag::new("b", "2")),
        );

        assert_eq!(cells[0].1, theme().tag(0));
        assert_eq!(cells[4].1, theme().tag(1));
    }

    #[test]
    fn mono_color_uses_one_style_for_every_tag() {
        let cells = spans(
            TagLine::mono_color()
                .tag(Tag::new("a", "1"))
                .tag(Tag::new("b", "2")),
        );

        assert_eq!(cells[0].1, theme().block_tag());
        assert_eq!(cells[4].1, theme().block_tag());
    }

    #[test]
    fn every_constructor_starts_left_aligned() {
        for line in [
            Line::from(TagLine::default()),
            Line::from(TagLine::multi_color()),
            Line::from(TagLine::mono_color()),
        ] {
            assert_eq!(line.alignment, Some(Alignment::Left));
        }
    }

    #[test]
    fn default_is_multi_color() {
        let line = Line::from(TagLine::default().tag(Tag::new("k", "v")));

        assert_eq!(line.spans[0].style, theme().tag(0));
    }

    #[test]
    fn alignment_is_configurable() {
        assert_eq!(
            Line::from(TagLine::multi_color().centered()).alignment,
            Some(Alignment::Center)
        );
        assert_eq!(
            Line::from(TagLine::multi_color().left_aligned()).alignment,
            Some(Alignment::Left)
        );
        assert_eq!(
            Line::from(TagLine::multi_color().right_aligned()).alignment,
            Some(Alignment::Right)
        );
    }

    #[test]
    fn owned_and_borrowed_text_are_both_accepted() {
        let owned = String::from("owned");
        let cells = spans(TagLine::multi_color().tag(Tag::new(owned, "borrowed")));

        assert_eq!(cells[0].0, " owned ");
        assert_eq!(cells[1].0, " borrowed ");
    }
}
