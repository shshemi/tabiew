use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Clrs;

impl SixColorsTwoRowsStyler for Clrs {
    const BACKGROUND: Color = Color::from_u32(0x00ffffff);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00262626);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00000000);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb0416),
        Color::from_u32(0x002cc631),
        Color::from_u32(0x00fdd727),
        Color::from_u32(0x001670ff),
        Color::from_u32(0x00e900b0),
        Color::from_u32(0x003ad5ce),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00f8282a),
        Color::from_u32(0x00328a5d),
        Color::from_u32(0x00fa701d),
        Color::from_u32(0x00135cd0),
        Color::from_u32(0x009f00bd),
        Color::from_u32(0x0033c3c1),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00FFFFFF), Color::from_u32(0x00FFFFFF)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x004FB3DC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00CA4000);
}
