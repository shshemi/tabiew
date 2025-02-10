use ratatui::style::{Color, Style, Stylize};

pub trait Styler {
    fn table_header() -> Style;
    fn table_header_cell(col: usize) -> Style;
    fn table_row(row: usize) -> Style;
    fn table_highlight() -> Style;
    fn sheet_value() -> Style;
    fn status_bar_error() -> Style;
    fn status_bar_prompt() -> Style;
    fn status_bar_search() -> Style;
    fn status_bar_info() -> Style;
    fn sheet_block() -> Style;
    fn status_bar_info_key(idx: usize) -> Style;
    fn status_bar_info_val(idx: usize) -> Style;
    fn highlight_info_key() -> Style;
    fn highlight_info_val() -> Style;
    fn pallete_text() -> Style;
    fn pallete_hightlight() -> Style;
    fn pallete() -> Style;
}
pub trait SixColorsTwoRowsStyler {
    const BACKGROUND: Color;
    const LIGHT_BACKGROUND: Color;
    const FOREGROUND: Color;

    const COLORS: [Color; 6];
    const DARK_COLORS: [Color; 6];

    const ROW_BACKGROUNDS: [Color; 2];
    const HIGHTLIGHT_BACKGROUND: Color;
    const HIGHTLIGHT_FOREGROUND: Color;

    const STATUS_BAR_ERROR: Color;
    const STATUS_BAR_PROMPT: Color;
    const STATUS_BAR_SEARCH: Color;
    const STATUS_BAR_INFO: Color;
}

impl<T> Styler for T
where
    T: SixColorsTwoRowsStyler,
{
    fn table_header() -> Style {
        Style::default().bg(Self::BACKGROUND)
    }

    fn table_header_cell(col: usize) -> Style {
        Style::default()
            .fg(Self::COLORS[col % Self::COLORS.len()])
            .bold()
    }

    fn table_row(row: usize) -> Style {
        Style::new()
            .bg(Self::ROW_BACKGROUNDS[row % Self::ROW_BACKGROUNDS.len()])
            .fg(Self::FOREGROUND)
    }

    fn table_highlight() -> Style {
        Style::new()
            .bg(Self::HIGHTLIGHT_BACKGROUND)
            .fg(Self::HIGHTLIGHT_FOREGROUND)
    }

    fn sheet_value() -> Style {
        Style::default().fg(Self::FOREGROUND)
    }

    fn status_bar_error() -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_ERROR)
            .fg(Self::FOREGROUND)
    }

    fn status_bar_prompt() -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_PROMPT)
            .fg(Self::FOREGROUND)
    }

    fn status_bar_search() -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_SEARCH)
            .fg(Self::FOREGROUND)
    }

    fn status_bar_info() -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_INFO)
            .fg(Self::FOREGROUND)
    }

    fn sheet_block() -> Style {
        Style::new()
            .bg(Self::BACKGROUND)
            .fg(Self::HIGHTLIGHT_BACKGROUND)
    }

    fn status_bar_info_key(idx: usize) -> Style {
        Style::default()
            .bg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn status_bar_info_val(idx: usize) -> Style {
        Style::default()
            .bg(Self::LIGHT_BACKGROUND)
            .fg(Self::COLORS[idx % Self::COLORS.len()])
    }

    fn pallete_text() -> Style {
        Style::default().bg(Self::BACKGROUND).fg(Self::FOREGROUND)
    }

    fn pallete_hightlight() -> Style {
        Style::default()
            .bg(Self::HIGHTLIGHT_BACKGROUND)
            .fg(Self::HIGHTLIGHT_FOREGROUND)
    }

    fn pallete() -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_INFO)
            .fg(Self::HIGHTLIGHT_BACKGROUND)
    }

    fn highlight_info_key() -> Style {
        Style::default()
            .bg(Self::HIGHTLIGHT_BACKGROUND)
            .fg(Self::HIGHTLIGHT_FOREGROUND)
    }

    fn highlight_info_val() -> Style {
        Style::default()
            .bg(Self::ROW_BACKGROUNDS[0])
            .fg(Self::HIGHTLIGHT_BACKGROUND)
    }
}

pub struct Monokai;
pub struct Argonaut;
pub struct Terminal;
pub struct Nord;
pub struct Catppuccin;
pub struct TokyoNight;

impl SixColorsTwoRowsStyler for Monokai {
    const BACKGROUND: Color = Color::from_u32(0x00141115);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003e3b3f);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);

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
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00c89f2d);
    const HIGHTLIGHT_FOREGROUND: Color = Self::BACKGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d02d00);
    const STATUS_BAR_PROMPT: Color = Color::from_u32(0x00109f2f);
    const STATUS_BAR_SEARCH: Color = Color::from_u32(0x00369aa6);
    const STATUS_BAR_INFO: Color = Self::BACKGROUND;
}

impl SixColorsTwoRowsStyler for Argonaut {
    const BACKGROUND: Color = Color::from_u32(0x0001030b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0023252d);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ff000f),
        Color::from_u32(0x00ffb900),
        Color::from_u32(0x00ffd866),
        Color::from_u32(0x008ce10b),
        Color::from_u32(0x006d43a6),
        Color::from_u32(0x0000d8eb),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0011131b), Color::from_u32(0x0001030b)];
    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00204a5b);
    const HIGHTLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00dd0000);
    const STATUS_BAR_PROMPT: Color = Color::from_u32(0x006cc100);
    const STATUS_BAR_SEARCH: Color = Color::from_u32(0x006f20eb);
    const STATUS_BAR_INFO: Color = Self::BACKGROUND;
}

impl SixColorsTwoRowsStyler for Nord {
    const BACKGROUND: Color = Color::from_u32(0x002E3440);

    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B4252);

    const FOREGROUND: Color = Color::from_u32(0x00ECEFF4);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00BF616A),
        Color::from_u32(0x00D08770),
        Color::from_u32(0x00EBCB8B),
        Color::from_u32(0x00A3BE8C),
        Color::from_u32(0x00B48EAD),
        Color::from_u32(0x005E81AC),
    ];

    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00AF515A),
        Color::from_u32(0x00C07760),
        Color::from_u32(0x00DBBB7B),
        Color::from_u32(0x0093AE7C),
        Color::from_u32(0x00A47E9D),
        Color::from_u32(0x004E719C),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x003B4252), Color::from_u32(0x00434C5E)];

    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00DBBB7B);

    const HIGHTLIGHT_FOREGROUND: Color = Color::from_u32(0x002E3440);

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF616A);

    const STATUS_BAR_PROMPT: Color = Color::from_u32(0x0093AE7C);

    const STATUS_BAR_SEARCH: Color = Color::from_u32(0x005E81AC);

    const STATUS_BAR_INFO: Color = Color::from_u32(0x002E3440);
}

impl SixColorsTwoRowsStyler for Catppuccin {
    const BACKGROUND: Color = Color::from_u32(0x0011111b);

    const LIGHT_BACKGROUND: Color = Color::from_u32(0x001e1e2e);

    const FOREGROUND: Color = Color::from_u32(0x00cdd6f4);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00cba6f7),
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00fab387),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x0074c7ec),
        Color::from_u32(0x0089b4fa),
    ];

    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00cba6f7),
        Color::from_u32(0x00f38ba8),
        Color::from_u32(0x00fab387),
        Color::from_u32(0x00a6e3a1),
        Color::from_u32(0x0074c7ec),
        Color::from_u32(0x0089b4fa),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00181825), Color::from_u32(0x001e1e2e)];

    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00f9e2af);

    const HIGHTLIGHT_FOREGROUND: Color = Color::from_u32(0x0011111b);

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d36b98);

    const STATUS_BAR_PROMPT: Color = Color::from_u32(0x0011111b);

    const STATUS_BAR_SEARCH: Color = Color::from_u32(0x004497bc);

    const STATUS_BAR_INFO: Color = Color::from_u32(0x0011111b);
}

impl SixColorsTwoRowsStyler for TokyoNight {
    const BACKGROUND: Color = Color::from_u32(0x001f2335);

    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00292e42);

    const FOREGROUND: Color = Color::from_u32(0x00dfe3f5);

    const COLORS: [Color; 6] = [
        Color::from_u32(0x00c53b53),
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00ff9e64),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x009d7cd8),
        Color::from_u32(0x0041a6b5),
    ];

    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00c53b53),
        Color::from_u32(0x00ff757f),
        Color::from_u32(0x00ff9e64),
        Color::from_u32(0x007aa2f7),
        Color::from_u32(0x009d7cd8),
        Color::from_u32(0x0041a6b5),
    ];

    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x00292e42), Color::from_u32(0x0024283b)];

    const HIGHTLIGHT_BACKGROUND: Color = Color::from_u32(0x00ffc777);

    const HIGHTLIGHT_FOREGROUND: Color = Color::from_u32(0x001f2335);

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00c53b53);

    const STATUS_BAR_PROMPT: Color = Color::from_u32(0x0041a6b5);

    const STATUS_BAR_SEARCH: Color = Color::from_u32(0x003d59a1);

    const STATUS_BAR_INFO: Color = Color::from_u32(0x001f2335);
}

impl Styler for Terminal {
    fn table_header() -> Style {
        Style::default().bg(Color::Cyan).fg(Color::Black)
    }

    fn table_header_cell(_col: usize) -> Style {
        Style::default()
    }

    fn table_row(_row: usize) -> Style {
        Default::default()
    }

    fn table_highlight() -> Style {
        Style::default().bg(Color::Yellow).fg(Color::Black)
    }

    fn sheet_value() -> Style {
        Style::default()
    }

    fn status_bar_error() -> Style {
        Style::default().bg(Color::Red).fg(Color::White)
    }

    fn status_bar_prompt() -> Style {
        Style::default().bg(Color::Green).fg(Color::White)
    }

    fn status_bar_search() -> Style {
        Style::default().bg(Color::Blue).fg(Color::White)
    }

    fn status_bar_info() -> Style {
        Style::default().bg(Color::Blue).fg(Color::White)
    }

    fn sheet_block() -> Style {
        Style::default()
    }

    fn status_bar_info_key(_idx: usize) -> Style {
        Style::default()
    }

    fn status_bar_info_val(_idx: usize) -> Style {
        Style::default()
    }

    fn pallete_text() -> Style {
        Style::default()
    }

    fn pallete_hightlight() -> Style {
        Style::default().bg(Color::Yellow).fg(Color::Black)
    }

    fn pallete() -> Style {
        Style::default()
    }

    fn highlight_info_key() -> Style {
        Style::default()
    }

    fn highlight_info_val() -> Style {
        Style::default()
    }
}
