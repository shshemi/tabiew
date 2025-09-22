use ratatui::style::Color;

use crate::tui::theme::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Monokai;

impl SixColorsTwoRowsStyler for Monokai {
    const BACKGROUND: Color = Color::from_u32(0x00141115);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003e3b3f);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005e5b5f);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff6188),
        Color::from_u32(0x00fc9867),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x00a9dc76),
        Color::from_u32(0x0078dce8),
        Color::from_u32(0x00ab9df2),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ee4066),
        Color::from_u32(0x00da7645),
        Color::from_u32(0x00ddb644),
        Color::from_u32(0x0087ba54),
        Color::from_u32(0x0056bac6),
        Color::from_u32(0x00897bd0),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00232024), Color::from_u32(0x001c191d)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00c89f2d);
    const HIGHLIGHT_FOREGROUND: Color = Self::BACKGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d02d00);

    fn id(&self) -> &str {
        "monokai"
    }

    fn title(&self) -> &str {
        "Monokai"
    }
}
