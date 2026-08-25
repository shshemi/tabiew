use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    widgets::Paragraph,
};

const MAX_WIDTH: u16 = 64;

pub trait RectExt {
    fn palette(self, height: u16) -> Self;
    fn toast(self, paragraph: &Paragraph) -> Self;
    fn popup(self, paragraph: &Paragraph) -> Self;
    fn goto_line(self, width: u16, height: u16) -> Self;
    fn plot(self) -> Self;
}

impl RectExt for Rect {
    fn palette(self, height: u16) -> Self {
        const WIDTH: u16 = 80;
        const MARGIN_TOP: u16 = 3;
        let [area] = Layout::horizontal([Constraint::Length(WIDTH)])
            .flex(Flex::Center)
            .areas(self);
        let [_, area] =
            Layout::vertical([Constraint::Length(MARGIN_TOP), Constraint::Length(height)])
                .areas(area);
        area
    }

    fn toast(self, paragraph: &Paragraph) -> Self {
        const MARGIN_BOTTOM: u16 = 3;
        let (width, height) = paragraph_size(paragraph);
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(self);
        let [area, _] = Layout::vertical([
            Constraint::Length(height),
            Constraint::Length(MARGIN_BOTTOM),
        ])
        .flex(Flex::End)
        .areas(area);
        area
    }

    fn popup(self, paragraph: &Paragraph) -> Self {
        let (width, height) = paragraph_size(paragraph);
        self.centered(Constraint::Length(width), Constraint::Length(height))
    }

    fn goto_line(self, width: u16, height: u16) -> Self {
        const MARGIN: u16 = 1;
        let [area, _] = Layout::horizontal([
            Constraint::Length(width),
            Constraint::Length(MARGIN),
        ])
        .flex(Flex::End)
        .areas(self);
        let [_, area] =
            Layout::vertical([Constraint::Length(MARGIN), Constraint::Length(height)]).areas(area);
        area
    }

    fn plot(self) -> Self {
        const MARGIN_HORIZONTAL: u16 = 7;
        const MARGIN_VERTICAL: u16 = 3;
        self.inner(Margin::new(MARGIN_HORIZONTAL, MARGIN_VERTICAL))
    }
}

fn paragraph_size(paragraph: &Paragraph) -> (u16, u16) {
    let width = paragraph.line_width().min(MAX_WIDTH as usize) as u16;
    let height = paragraph.line_count(width) as u16;
    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod palette {
        use super::*;

        #[test]
        fn is_horizontally_centered_at_a_fixed_width() {
            let area = Rect::new(0, 0, 100, 30).palette(5);
            assert_eq!(area.width, 80);
            assert_eq!(area.x, 10);
        }

        #[test]
        fn hangs_below_a_fixed_top_margin() {
            let area = Rect::new(0, 0, 100, 30).palette(5);
            assert_eq!(area.y, 3);
            assert_eq!(area.height, 5);
        }

        #[test]
        fn honours_the_origin_of_the_source_rect() {
            let area = Rect::new(4, 6, 100, 30).palette(5);
            assert_eq!(area.x, 14);
            assert_eq!(area.y, 9);
        }
    }

    mod toast {
        use super::*;
        use ratatui::widgets::Wrap;

        fn paragraph(text: String) -> Paragraph<'static> {
            Paragraph::new(text).wrap(Wrap { trim: true })
        }

        #[test]
        fn short_content_keeps_its_own_width() {
            let area = Rect::new(0, 0, 100, 30).toast(&paragraph("hello".to_owned()));
            assert_eq!(area.width, 5);
        }

        #[test]
        fn wide_content_is_capped() {
            let area = Rect::new(0, 0, 100, 30).toast(&paragraph("w".repeat(90)));
            assert_eq!(area.width, 64);
        }

        #[test]
        fn is_horizontally_centered() {
            let area = Rect::new(0, 0, 100, 30).toast(&paragraph("w".repeat(90)));
            assert_eq!(area.x, 18);
        }

        #[test]
        fn height_follows_the_wrapped_line_count() {
            let one = Rect::new(0, 0, 100, 30).toast(&paragraph("hello".to_owned()));
            let many = Rect::new(0, 0, 100, 30).toast(&paragraph("w ".repeat(80)));

            assert_eq!(one.height, 1);
            assert!(many.height > 1);
        }

        #[test]
        fn sits_above_a_fixed_bottom_margin() {
            let area = Rect::new(0, 0, 100, 30).toast(&paragraph("hello".to_owned()));
            assert_eq!(area.bottom(), 30 - 3);
        }

        #[test]
        fn taller_content_grows_upward() {
            let short = Rect::new(0, 0, 100, 30).toast(&paragraph("hello".to_owned()));
            let tall = Rect::new(0, 0, 100, 30).toast(&paragraph("w ".repeat(80)));

            assert_eq!(short.bottom(), tall.bottom());
            assert!(tall.y < short.y);
        }

        #[test]
        fn honours_the_origin_of_the_source_rect() {
            let area = Rect::new(4, 6, 100, 30).toast(&paragraph("hello".to_owned()));
            assert_eq!(area.x, 52);
            assert_eq!(area.bottom(), 36 - 3);
        }
    }

    mod popup {
        use super::*;
        use ratatui::widgets::Wrap;

        fn paragraph(text: String) -> Paragraph<'static> {
            Paragraph::new(text).wrap(Wrap { trim: true })
        }

        #[test]
        fn short_content_keeps_its_own_width() {
            let area = Rect::new(0, 0, 100, 30).popup(&paragraph("hello".to_owned()));
            assert_eq!(area.width, 5);
        }

        #[test]
        fn wide_content_is_capped() {
            let area = Rect::new(0, 0, 100, 30).popup(&paragraph("w".repeat(90)));
            assert_eq!(area.width, 64);
        }

        #[test]
        fn is_centered_on_both_axes() {
            let area = Rect::new(0, 0, 100, 30).popup(&paragraph("w".repeat(90)));

            assert_eq!(area.x, 100 - area.right());
            assert_eq!(area.y, 30 - area.bottom());
        }

        #[test]
        fn taller_content_grows_in_both_directions() {
            let short = Rect::new(0, 0, 100, 30).popup(&paragraph("hello".to_owned()));
            let tall = Rect::new(0, 0, 100, 30).popup(&paragraph("w ".repeat(80)));

            assert!(tall.y < short.y);
            assert!(tall.bottom() > short.bottom());
        }

        #[test]
        fn honours_the_origin_of_the_source_rect() {
            let plain = Rect::new(0, 0, 100, 30).popup(&paragraph("hello".to_owned()));
            let shifted = Rect::new(4, 6, 100, 30).popup(&paragraph("hello".to_owned()));

            assert_eq!(shifted.x, plain.x + 4);
            assert_eq!(shifted.y, plain.y + 6);
        }
    }

    mod goto_line {
        use super::*;

        #[test]
        fn hugs_the_right_edge_with_a_one_cell_margin() {
            let area = Rect::new(0, 0, 100, 30).goto_line(32, 3);

            assert_eq!(area.width, 32);
            assert_eq!(area.right(), 99);
        }

        #[test]
        fn hangs_below_a_one_cell_top_margin() {
            let area = Rect::new(0, 0, 100, 30).goto_line(32, 3);

            assert_eq!(area.y, 1);
            assert_eq!(area.height, 3);
        }

        #[test]
        fn honours_the_origin_of_the_source_rect() {
            let area = Rect::new(4, 6, 100, 30).goto_line(32, 3);

            assert_eq!(area.right(), 103);
            assert_eq!(area.y, 7);
        }
    }

    mod plot {
        use super::*;

        #[test]
        fn insets_the_area_on_every_side() {
            let area = Rect::new(0, 0, 100, 30).plot();

            assert_eq!(area, Rect::new(7, 3, 86, 24));
        }

        #[test]
        fn an_area_smaller_than_the_margin_collapses() {
            let area = Rect::new(0, 0, 4, 2).plot();

            assert_eq!(area.width, 0);
            assert_eq!(area.height, 0);
        }
    }
}
