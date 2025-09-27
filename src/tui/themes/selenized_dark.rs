
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SelenizedDark;

impl SixColorsTwoRowsStyler for SelenizedDark {
    const BACKGROUND: Color = Color::from_u32(0x00103c48);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00305C68);
    const FOREGROUND: Color = Color::from_u32(0x00adbcbc);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00184956);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff665c),
        Color::from_u32(0x0084c747),
        Color::from_u32(0x00ebc13d),
        Color::from_u32(0x0058a3ff),
        Color::from_u32(0x00ff84cd),
        Color::from_u32(0x0053d6c7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00fa5750),
        Color::from_u32(0x0075b938),
        Color::from_u32(0x00dbb32d),
        Color::from_u32(0x004695f7),
        Color::from_u32(0x00f275be),
        Color::from_u32(0x0041c7b9),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0016424E), Color::from_u32(0x001C4854)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x008D9C9C);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA2720);

    fn id(&self) -> &str {
        "selenized_dark"
    }

    fn title(&self) -> &str {
        "SelenizedDark"
    }
}
