use ratatui::style::Color;

pub trait ColorExt {
    fn darken(&self) -> Self;
}

impl ColorExt for Color {
    fn darken(&self) -> Self {
        match self {
            Color::Rgb(r, g, b) => Color::Rgb(
                (*r as f32 * 0.2) as u8,
                (*g as f32 * 0.2) as u8,
                (*b as f32 * 0.2) as u8,
            ),
            _ => *self,
        }
    }
}
