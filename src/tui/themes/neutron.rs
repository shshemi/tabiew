use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Neutron;

impl SixColorsTwoRowsStyler for Neutron {
    const BACKGROUND: Color = Color::from_u32(0x001c1e22);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003C3E42);
    const FOREGROUND: Color = Color::from_u32(0x00e6e8ef);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0023252b);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00b54036),
        Color::from_u32(0x005ab977),
        Color::from_u32(0x00deb566),
        Color::from_u32(0x006a7c93),
        Color::from_u32(0x00a4799d),
        Color::from_u32(0x003f94a8),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00b54036),
        Color::from_u32(0x005ab977),
        Color::from_u32(0x00deb566),
        Color::from_u32(0x006a7c93),
        Color::from_u32(0x00a4799d),
        Color::from_u32(0x003f94a8),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00222428), Color::from_u32(0x00282A2E)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00D6D7CC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00AE8536);
}
