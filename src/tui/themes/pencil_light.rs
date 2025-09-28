
use ratatui::style::Color;

use crate::tui::themes::styler::SixColorsTwoRowsStyler;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PencilLight;

impl SixColorsTwoRowsStyler for PencilLight {
    const BACKGROUND: Color = Color::from_u32(0x00f1f1f1);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00FFFFFF);
    const FOREGROUND: Color = Color::from_u32(0x00424242);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00212121);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00fb007a),
        Color::from_u32(0x005fd7af),
        Color::from_u32(0x00f3e430),
        Color::from_u32(0x0020bbfc),
        Color::from_u32(0x006855de),
        Color::from_u32(0x004fb8cc),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c30771),
        Color::from_u32(0x0010a778),
        Color::from_u32(0x00a89c14),
        Color::from_u32(0x00008ec4),
        Color::from_u32(0x00523c79),
        Color::from_u32(0x0020a5ba),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00F7F7F7), Color::from_u32(0x00FDFDFD)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00009BDC);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00930041);
}
