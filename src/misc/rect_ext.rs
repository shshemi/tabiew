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
        let [area, _] = Layout::horizontal([Constraint::Length(width), Constraint::Length(MARGIN)])
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
