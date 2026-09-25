use ratatui::text::{Line, Span};

#[derive(Debug, Default)]
pub struct LineBuilder {
    buf: Vec<Span<'static>>,
}

impl LineBuilder {
    pub fn push_span(&mut self, span: Span<'static>) {
        self.buf.push(span);
    }

    pub fn into_line(self) -> Line<'static> {
        self.buf.into()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}
