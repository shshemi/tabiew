use ratatui::style::{Color, Style, Stylize};
use serde::{Deserialize, Serialize};

pub trait Styler {
    fn table_header(&self) -> Style;
    fn row(&self, row: usize) -> Style;
    fn highlight(&self) -> Style;
    fn header(&self, col: usize) -> Style;
    fn tag(&self, col: usize) -> Style;
    fn block(&self) -> Style;
    fn block_tag(&self) -> Style;
    fn text(&self) -> Style;
    fn subtext(&self) -> Style;
    fn error(&self) -> Style;
    fn graph(&self, idx: usize) -> Style;
}

pub trait SixColorsTwoRowsStyler {
    const BACKGROUND: Color;
    const LIGHT_BACKGROUND: Color;
    const FOREGROUND: Color;
    const DARK_FOREGROUND: Color;

    const COLORS: [Color; 6];
    const DARK_COLORS: [Color; 6];

    const ROW_BACKGROUNDS: [Color; 2];
    const HIGHLIGHT_BACKGROUND: Color;
    const HIGHLIGHT_FOREGROUND: Color;

    const STATUS_BAR_ERROR: Color;
}

impl<T> Styler for T
where
    T: SixColorsTwoRowsStyler,
{
    fn table_header(&self) -> Style {
        Style::default().bg(Self::BACKGROUND)
    }

    fn header(&self, idx: usize) -> Style {
        Style::default()
            .fg(Self::COLORS[idx % Self::COLORS.len()])
            .bold()
    }

    fn row(&self, idx: usize) -> Style {
        Style::new()
            .bg(Self::ROW_BACKGROUNDS[idx % Self::ROW_BACKGROUNDS.len()])
            .fg(Self::FOREGROUND)
    }

    fn highlight(&self) -> Style {
        Style::new()
            .bg(Self::HIGHLIGHT_BACKGROUND)
            .fg(Self::HIGHLIGHT_FOREGROUND)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn block(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::HIGHLIGHT_BACKGROUND)
    }

    fn block_tag(&self) -> Style {
        Style::default()
            .bg(Self::HIGHLIGHT_BACKGROUND)
            .fg(Self::LIGHT_BACKGROUND)
    }

    fn text(&self) -> Style {
        Style::default().bg(Self::BACKGROUND).fg(Self::FOREGROUND)
    }

    fn subtext(&self) -> Style {
        Style::default()
            .bg(Self::BACKGROUND)
            .fg(Self::DARK_FOREGROUND)
    }

    fn error(&self) -> Style {
        Style::default()
            .bg(Self::STATUS_BAR_ERROR)
            .fg(Self::FOREGROUND)
    }

    fn graph(&self, idx: usize) -> Style {
        Style::default()
            .fg(Self::DARK_COLORS[idx % Self::DARK_COLORS.len()])
            .bold()
    }
}

#[derive(Debug, Default)]
pub struct Monokai;

#[derive(Debug, Default)]
pub struct Argonaut;

#[derive(Debug, Default)]
pub struct Terminal;

#[derive(Debug, Default)]
pub struct Nord;

#[derive(Debug, Default)]
pub struct Catppuccin;

#[derive(Debug, Default)]
pub struct TokyoNight;

#[derive(Debug, Default)]
pub struct Chakra;

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
}

impl SixColorsTwoRowsStyler for Argonaut {
    const BACKGROUND: Color = Color::from_u32(0x0001030b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x0023252d);
    const FOREGROUND: Color = Color::from_u32(0x00fffaf4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x0043454d);

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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00204a5b);
    const HIGHLIGHT_FOREGROUND: Color = Self::FOREGROUND;

    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00dd0000);
}

impl SixColorsTwoRowsStyler for Nord {
    const BACKGROUND: Color = Color::from_u32(0x002E3440);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x003B4252);
    const FOREGROUND: Color = Color::from_u32(0x00ECEFF4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x005B6272);

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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00DBBB7B);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x002E3440);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00BF616A);
}

impl SixColorsTwoRowsStyler for Catppuccin {
    const BACKGROUND: Color = Color::from_u32(0x0011111b);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x001e1e2e);
    const FOREGROUND: Color = Color::from_u32(0x00cdd6f4);
    const DARK_FOREGROUND: Color = Color::from_u32(0x003e3e4e);

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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00f9e2af);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x0011111b);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00d36b98);
}

impl SixColorsTwoRowsStyler for TokyoNight {
    const BACKGROUND: Color = Color::from_u32(0x001f2335);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00292e42);
    const FOREGROUND: Color = Color::from_u32(0x00dfe3f5);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00494e62);

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
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ffc777);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x001f2335);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00c53b53);
}

impl SixColorsTwoRowsStyler for Chakra {
    const BACKGROUND: Color = Color::from_u32(0x00111111);
    const LIGHT_BACKGROUND: Color = Color::from_u32(0x00191917);
    const FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const DARK_FOREGROUND: Color = Color::from_u32(0x00a1a1aa);
    const COLORS: [Color; 6] = [
        Color::from_u32(0x00f472b6),
        Color::from_u32(0x00c084fc),
        Color::from_u32(0x0022d3ee),
        Color::from_u32(0x0060a5fa),
        Color::from_u32(0x002dd4bf),
        Color::from_u32(0x004ade80),
    ];
    const DARK_COLORS: [Color; 6] = [
        Color::from_u32(0x00ec4899),
        Color::from_u32(0x00a855f7),
        Color::from_u32(0x0006b6d4),
        Color::from_u32(0x003b82f6),
        Color::from_u32(0x0014b8a6),
        Color::from_u32(0x0022c55e),
    ];
    const ROW_BACKGROUNDS: [Color; 2] = [Color::from_u32(0x0018181b), Color::from_u32(0x00111111)];
    const HIGHLIGHT_BACKGROUND: Color = Color::from_u32(0x00ca8a04);
    const HIGHLIGHT_FOREGROUND: Color = Color::from_u32(0x00fafafa);
    const STATUS_BAR_ERROR: Color = Color::from_u32(0x00991919);
}

impl Styler for Terminal {
    fn table_header(&self) -> Style {
        Style::default().bg(Color::DarkGray).fg(Color::White)
    }

    fn row(&self, _row: usize) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn highlight(&self) -> Style {
        Style::default().bg(Color::LightYellow).fg(Color::Black)
    }

    fn header(&self, _col: usize) -> Style {
        Style::default().fg(Color::White)
    }

    fn tag(&self, idx: usize) -> Style {
        Style::default()
            .bg([
                Color::Red,
                Color::Magenta,
                Color::Blue,
                Color::Cyan,
                Color::Green,
            ][idx % 5])
            .fg(Color::Gray)
    }

    fn block_tag(&self) -> Style {
        Style::default().bg(Color::LightYellow).fg(Color::Gray)
    }

    fn block(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn text(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::White)
    }

    fn subtext(&self) -> Style {
        Style::default().bg(Color::Black).fg(Color::DarkGray)
    }

    fn error(&self) -> Style {
        Style::default().bg(Color::Red).fg(Color::White)
    }

    fn graph(&self, _idx: usize) -> Style {
        Style::default().fg(Color::White)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    table_header: Style,
    table_headers: Vec<Style>,
    rows: Vec<Style>,
    highlight: Style,
    table_tags: Vec<Style>,
    block: Style,
    block_tag: Style,
    text: Style,
    subtext: Style,
    error: Style,
    chart: Vec<Style>,
}

impl Theme {
    pub fn sample() -> Self {
        Theme {
            table_header: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffffff)),
            table_headers: vec![
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ffff00)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ff00ff)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x0000ffff)),
            ],
            rows: vec![
                Style::default()
                    .bg(Color::from_u32(0x00383838))
                    .fg(Color::from_u32(0x00ffffff)),
                Style::default()
                    .bg(Color::from_u32(0x00101010))
                    .fg(Color::from_u32(0x00ffffff)),
            ],
            highlight: Style::default()
                .bg(Color::from_u32(0x00ffff00))
                .fg(Color::from_u32(0x00000000)),
            table_tags: vec![
                Style::default()
                    .bg(Color::from_u32(0x00ff0000))
                    .fg(Color::from_u32(0x00000000)),
                Style::default()
                    .bg(Color::from_u32(0x0000ff00))
                    .fg(Color::from_u32(0x00000000)),
            ],
            block: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffff00)),
            block_tag: Style::default()
                .bg(Color::from_u32(0x00ffff00))
                .fg(Color::from_u32(0x00000000)),
            text: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ffffff)),
            subtext: Style::default()
                .bg(Color::from_u32(0x00000000))
                .fg(Color::from_u32(0x00ababab)),
            error: Style::default()
                .bg(Color::from_u32(0x00ff0000))
                .fg(Color::from_u32(0x00ffffff)),
            chart: vec![
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ffff00)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x00ff00ff)),
                Style::default()
                    .bg(Color::from_u32(0x00000000))
                    .fg(Color::from_u32(0x0000ffff)),
            ],
        }
    }
}

impl super::Styler for Theme {
    fn table_header(&self) -> Style {
        self.table_header
    }

    fn header(&self, idx: usize) -> Style {
        self.table_headers[idx % self.table_headers.len()]
    }

    fn row(&self, idx: usize) -> Style {
        self.rows[idx % self.rows.len()]
    }

    fn highlight(&self) -> Style {
        self.highlight
    }

    fn tag(&self, idx: usize) -> Style {
        self.table_tags[idx % self.table_tags.len()]
    }

    fn block(&self) -> Style {
        self.block
    }

    fn block_tag(&self) -> Style {
        self.block_tag
    }

    fn text(&self) -> Style {
        self.text
    }

    fn subtext(&self) -> Style {
        self.subtext
    }

    fn error(&self) -> Style {
        self.error
    }

    fn graph(&self, idx: usize) -> Style {
        self.chart[idx % self.chart.len()]
    }
}
