use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    symbols::line,
    widgets::{Block, Widget},
};

use crate::misc::config::theme;

#[derive(Debug)]
pub struct Split<'a, const N: usize> {
    direction: Direction,
    constraints: [Constraint; N],
    block: Option<Block<'a>>,
    style: Style,
}

impl<'a, const N: usize> Split<'a, N> {
    pub fn horizontal(constraints: [Constraint; N]) -> Self {
        Self::new(Direction::Horizontal, constraints)
    }

    pub fn vertical(constraints: [Constraint; N]) -> Self {
        Self::new(Direction::Vertical, constraints)
    }

    fn new(direction: Direction, constraints: [Constraint; N]) -> Self {
        Self {
            direction,
            constraints,
            block: None,
            style: theme().block(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    fn layout(&self) -> Layout {
        Layout::default()
            .direction(self.direction)
            .constraints(self.constraints)
            .spacing(1)
    }

    fn inner(&self, area: Rect) -> Rect {
        self.block.as_ref().map_or(area, |block| block.inner(area))
    }

    pub fn split(self, buf: &mut Buffer, area: Rect) -> [Rect; N] {
        let direction = self.direction;
        let style = self.style;
        let inner = self.inner(area);
        let layout = self.layout();
        let has_block = self.block.is_some();

        if let Some(block) = self.block {
            block.render(area, buf);
        }

        let (segments, spacers) = layout.split_with_spacers(inner);

        for spacer in spacers.iter().skip(1).take(spacers.len().saturating_sub(2)) {
            draw_divider(direction, *spacer, area, has_block, style, buf);
        }

        segments
            .as_ref()
            .try_into()
            .expect("layout constraints define exactly N segments")
    }
}

fn draw_divider(
    direction: Direction,
    spacer: Rect,
    outer: Rect,
    has_block: bool,
    style: Style,
    buf: &mut Buffer,
) {
    match direction {
        Direction::Horizontal => {
            for y in spacer.top()..spacer.bottom() {
                buf.set_string(spacer.x, y, line::NORMAL.vertical, style);
            }
            if has_block {
                buf.set_string(spacer.x, outer.top(), line::NORMAL.horizontal_down, style);
                buf.set_string(
                    spacer.x,
                    outer.bottom() - 1,
                    line::NORMAL.horizontal_up,
                    style,
                );
            }
        }
        Direction::Vertical => {
            for x in spacer.left()..spacer.right() {
                buf.set_string(x, spacer.y, line::NORMAL.horizontal, style);
            }
            if has_block {
                buf.set_string(outer.left(), spacer.y, line::NORMAL.vertical_right, style);
                buf.set_string(
                    outer.right() - 1,
                    spacer.y,
                    line::NORMAL.vertical_left,
                    style,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::widgets::BorderType;

    fn cell_symbol(buf: &Buffer, x: u16, y: u16) -> &str {
        buf[(x, y)].symbol()
    }

    #[test]
    fn split_leaves_one_cell_gaps_horizontal() {
        let area = Rect::new(0, 0, 11, 5);
        let mut buf = Buffer::empty(area);
        let areas = Split::horizontal([Constraint::Length(5), Constraint::Length(5)])
            .split(&mut buf, area);

        assert_eq!(areas, [Rect::new(0, 0, 5, 5), Rect::new(6, 0, 5, 5)]);
    }

    #[test]
    fn split_leaves_one_cell_gaps_vertical() {
        let area = Rect::new(0, 0, 5, 11);
        let mut buf = Buffer::empty(area);
        let areas =
            Split::vertical([Constraint::Length(5), Constraint::Length(5)]).split(&mut buf, area);

        assert_eq!(areas, [Rect::new(0, 0, 5, 5), Rect::new(0, 6, 5, 5)]);
    }

    #[test]
    fn split_accounts_for_block_border() {
        let area = Rect::new(0, 0, 13, 7);
        let mut buf = Buffer::empty(area);
        let areas = Split::horizontal([Constraint::Length(5), Constraint::Length(5)])
            .block(Block::bordered())
            .split(&mut buf, area);

        assert_eq!(areas, [Rect::new(1, 1, 5, 5), Rect::new(7, 1, 5, 5)]);
    }

    #[test]
    fn split_without_block_draws_plain_divider_no_junctions() {
        let area = Rect::new(0, 0, 11, 3);
        let mut buf = Buffer::empty(area);
        let areas = Split::horizontal([Constraint::Length(5), Constraint::Length(5)])
            .split(&mut buf, area);

        assert_eq!(areas, [Rect::new(0, 0, 5, 3), Rect::new(6, 0, 5, 3)]);
        for y in 0..3 {
            assert_eq!(cell_symbol(&buf, 5, y), "│");
        }
    }

    #[test]
    fn split_horizontal_with_block_joins_divider_into_border() {
        let area = Rect::new(0, 0, 13, 5);
        let mut buf = Buffer::empty(area);
        let areas = Split::horizontal([Constraint::Length(5), Constraint::Length(5)])
            .block(Block::bordered())
            .split(&mut buf, area);

        assert_eq!(areas, [Rect::new(1, 1, 5, 5 - 2), Rect::new(7, 1, 5, 5 - 2)]);

        let divider_x = 6;
        assert_eq!(cell_symbol(&buf, divider_x, 0), "┬");
        assert_eq!(cell_symbol(&buf, divider_x, 4), "┴");
        for y in 1..4 {
            assert_eq!(cell_symbol(&buf, divider_x, y), "│");
        }
    }

    #[test]
    fn split_vertical_with_block_joins_divider_into_border() {
        let area = Rect::new(0, 0, 5, 13);
        let mut buf = Buffer::empty(area);
        let areas = Split::vertical([Constraint::Length(5), Constraint::Length(5)])
            .block(Block::bordered())
            .split(&mut buf, area);

        assert_eq!(areas, [Rect::new(1, 1, 3, 5), Rect::new(1, 7, 3, 5)]);

        let divider_y = 6;
        assert_eq!(cell_symbol(&buf, 0, divider_y), "├");
        assert_eq!(cell_symbol(&buf, 4, divider_y), "┤");
        for x in 1..4 {
            assert_eq!(cell_symbol(&buf, x, divider_y), "─");
        }
    }

    #[test]
    fn single_segment_draws_no_divider() {
        let area = Rect::new(0, 0, 10, 5);
        let mut buf = Buffer::empty(area);
        let areas = Split::horizontal([Constraint::Fill(1)]).split(&mut buf, area);

        assert_eq!(areas, [area]);
        assert!(
            buf.content()
                .iter()
                .all(|c| c.symbol() != "│" && c.symbol() != "┬")
        );
    }

    #[test]
    fn style_defaults_to_block_theme_and_is_overridable() {
        let area = Rect::new(0, 0, 11, 3);
        let mut buf = Buffer::empty(area);
        Split::horizontal([Constraint::Length(5), Constraint::Length(5)])
            .style(Style::default().fg(ratatui::style::Color::Red))
            .split(&mut buf, area);

        assert_eq!(
            buf[(5, 0)].fg,
            ratatui::style::Color::Red,
            "explicit style overrides the theme default"
        );
    }

    #[test]
    fn block_renders_with_border_type() {
        let area = Rect::new(0, 0, 13, 5);
        let mut buf = Buffer::empty(area);
        Split::horizontal([Constraint::Length(5), Constraint::Length(5)])
            .block(Block::bordered().border_type(BorderType::Double))
            .split(&mut buf, area);

        assert_eq!(cell_symbol(&buf, 0, 0), "╔");
    }
}
