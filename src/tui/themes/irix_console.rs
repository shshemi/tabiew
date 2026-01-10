use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IrixConsole;

impl SixColorsTwoRowsStyler for IrixConsole {
    const BACKGROUND: Color = Color::from_u32(0x000c0c0c);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x002C2C2C);
    const FOREGROUND: Color = Color::from_u32(0x00f2f2f2);
    const DARK_FOREGROUND: Color = Color::from_u32(0x001a1919);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f34f59),
        Color::from_u32(0x0045c731),
        Color::from_u32(0x00f9f2a7),
        Color::from_u32(0x004079ff),
        Color::from_u32(0x00c31ba2),
        Color::from_u32(0x006ed7d7),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00d42426),
        Color::from_u32(0x0037a327),
        Color::from_u32(0x00c29d28),
        Color::from_u32(0x000739e2),
        Color::from_u32(0x00911f9c),
        Color::from_u32(0x004497df),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00121212), Color::from_u32(0x00181818)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00A7A7A7);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00A40000);
}
