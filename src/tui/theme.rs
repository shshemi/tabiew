use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use ratatui::style::{Color, Style, Stylize};
use serde::{Deserialize, Serialize};

use crate::{
    args::AppTheme,
    tui::themes::{
        argonaut::Argonaut, catppuccin::Catppuccin, chakra::Chakra, custom::Custom,
        monokai::Monokai, nord::Nord, terminal::Terminal, tokyo_night::TokyoNight,
    },
};

#[derive(Debug, Clone)]
pub struct Theme {
    styler: Option<Arc<dyn Styler + Send + Sync>>,
}

impl Theme {
    fn new<S: Styler + Send + Sync + 'static>(theme: S) -> Self {
        Theme {
            styler: Some(Arc::new(theme)),
        }
    }

    fn styler(&self) -> Arc<dyn Styler> {
        self.styler.clone().unwrap_or(Arc::new(Monokai))
    }

    pub fn all() -> &'static [Theme] {
        static ALL: OnceLock<Vec<Theme>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                Theme::new(Monokai),
                Theme::new(Argonaut),
                Theme::new(Nord),
                Theme::new(Catppuccin),
                Theme::new(TokyoNight),
                Theme::new(Chakra),
                Theme::new(Terminal),
                Theme::new(Custom::default()),
            ]
        })
    }

    pub const fn default() -> Self {
        Self { styler: None }
    }
}

impl From<AppTheme> for Theme {
    fn from(value: AppTheme) -> Self {
        match value {
            AppTheme::Monokai => Theme::new(Monokai),
            AppTheme::Argonaut => Theme::new(Argonaut),
            AppTheme::Nord => Theme::new(Nord),
            AppTheme::Catppuccin => Theme::new(Catppuccin),
            AppTheme::TokyoNight => Theme::new(TokyoNight),
            AppTheme::Terminal => Theme::new(Terminal),
            AppTheme::Chakra => Theme::new(Chakra),
            AppTheme::Config => Theme::new(Custom::default()),
        }
    }
}

impl Styler for Theme {
    fn table_header(&self) -> Style {
        self.styler().table_header()
    }

    fn row(&self, row: usize) -> Style {
        self.styler().row(row)
    }

    fn highlight(&self) -> Style {
        self.styler().highlight()
    }

    fn header(&self, col: usize) -> Style {
        self.styler().header(col)
    }

    fn tag(&self, col: usize) -> Style {
        self.styler().tag(col)
    }

    fn block(&self) -> Style {
        self.styler().block()
    }

    fn block_tag(&self) -> Style {
        self.styler().block_tag()
    }

    fn text(&self) -> Style {
        self.styler().text()
    }

    fn subtext(&self) -> Style {
        self.styler().subtext()
    }

    fn error(&self) -> Style {
        self.styler().error()
    }

    fn graph(&self, idx: usize) -> Style {
        self.styler().graph(idx)
    }

    fn id(&self) -> &str {
        if let Some(inner) = self.styler.as_ref() {
            inner.id()
        } else {
            "monokai"
        }
    }

    fn title(&self) -> &str {
        if let Some(inner) = self.styler.as_ref() {
            inner.title()
        } else {
            "Monokai"
        }
    }
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.styler().id() == other.styler().id()
    }
}

impl Eq for Theme {}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let theme_str = String::deserialize(deserializer)?;
        match theme_str.as_str() {
            "monokai" => Ok(Theme::new(Monokai)),
            "argonaut" => Ok(Theme::new(Monokai)),
            "terminal" => Ok(Theme::new(Monokai)),
            "nord" => Ok(Theme::new(Monokai)),
            "catppuccin" => Ok(Theme::new(Monokai)),
            "tokyo-night" => Ok(Theme::new(Monokai)),
            "chakra" => Ok(Theme::new(Monokai)),
            "custom" => Ok(Theme::new(
                Custom::read_from_config_dir().unwrap_or_default(),
            )),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown theme: {}",
                theme_str
            ))),
        }
    }
}

pub trait Styler: Debug {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
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
    fn id(&self) -> &str;
    fn title(&self) -> &str;
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
    T: SixColorsTwoRowsStyler + Debug,
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

    fn id(&self) -> &str {
        self.id()
    }

    fn title(&self) -> &str {
        self.title()
    }
}
