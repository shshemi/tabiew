use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    style::Stylize,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{
    misc::globals::theme,
    tui::{
        component::Component,
        status_bar::{StatusBar, Tag},
        widgets::block::Block,
    },
};

#[derive(Debug)]
pub struct Help {}

impl Help {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Help {
    fn render(
        &mut self,
        _area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _focus_state: crate::tui::component::FocusState,
    ) {
        let [area] = Layout::horizontal([Constraint::Length(90)])
            .flex(Flex::Center)
            .areas(buf.area);
        let [_, area] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(50)]).areas(area);

        Clear.render(area, buf);

        let mut lines = vec![
            Line::raw(""),
            Line::from(vec![Span::styled(
                "Tabiew Keyboard Shortcuts",
                theme().header(0),
            )]),
            Line::raw(""),
        ];

        // Navigation
        lines.push(Line::styled("Navigation", theme().header(1)));
        lines.push(Line::from(vec![
            Span::styled("  h j k l", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("← ↓ ↑ →", theme().text().bold()),
            Span::raw("     Navigate"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  b", theme().text().bold()),
            Span::raw(" / "),
            Span::styled("w", theme().text().bold()),
            Span::raw("                 Previous / next column"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Ctrl + u", theme().text().bold()),
            Span::raw(" / "),
            Span::styled("Ctrl + d", theme().text().bold()),
            Span::raw("   Move half page up/down"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Ctrl + b", theme().text().bold()),
            Span::raw(" / "),
            Span::styled("Ctrl + f", theme().text().bold()),
            Span::raw("   Move full page up/down"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Home", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("g", theme().text().bold()),
            Span::raw("             Move to first row"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  End", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("G", theme().text().bold()),
            Span::raw("              Move to last row"),
        ]));
        lines.push(Line::raw(""));

        // View Actions
        lines.push(Line::styled("View Actions", theme().header(1)));
        lines.push(Line::from(vec![
            Span::styled("  Enter", theme().text().bold()),
            Span::raw("                  Open sheet (cell detail view)"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  e", theme().text().bold()),
            Span::raw("                       Toggle Auto-Fit"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  f", theme().text().bold()),
            Span::raw("                       Toggle Borders"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  I", theme().text().bold()),
            Span::raw("                       Show DataFrame Info"),
        ]));
        lines.push(Line::raw(""));

        // Search
        lines.push(Line::styled("Search", theme().header(1)));
        lines.push(Line::from(vec![
            Span::styled("  /", theme().text().bold()),
            Span::raw("                       Fuzzy Search"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  ?", theme().text().bold()),
            Span::raw("                       Exact Search"),
        ]));
        lines.push(Line::raw(""));

        // Commands
        lines.push(Line::styled("Commands", theme().header(1)));
        lines.push(Line::from(vec![
            Span::styled("  :", theme().text().bold()),
            Span::raw("                       Command Palette"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Ctrl + r", theme().text().bold()),
            Span::raw("              Reset data frame"),
        ]));
        lines.push(Line::raw(""));

        // Tabs
        lines.push(Line::styled("Tabs", theme().header(1)));
        lines.push(Line::from(vec![
            Span::styled("  H", theme().text().bold()),
            Span::raw(" / "),
            Span::styled("L", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("Shift+←", theme().text().bold()),
            Span::raw(" / "),
            Span::styled("Shift+→", theme().text().bold()),
            Span::raw("   Previous / next tab"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  t", theme().text().bold()),
            Span::raw("                       Show tab panel"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  1-9", theme().text().bold()),
            Span::raw("                     Go to tab number"),
        ]));
        lines.push(Line::raw(""));

        // Quit
        lines.push(Line::styled("Quit", theme().header(1)));
        lines.push(Line::from(vec![
            Span::styled("  q", theme().text().bold()),
            Span::raw("                       Close current view/tab"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Q", theme().text().bold()),
            Span::raw("                       Quit Application"),
        ]));
        lines.push(Line::raw(""));

        // Common Commands
        lines.push(Line::styled(
            "Common Commands (after pressing :)",
            theme().header(1),
        ));
        lines.push(Line::from(vec![
            Span::styled("  Q", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("query", theme().text().bold()),
            Span::raw("           Query data with SQL"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  S", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("select", theme().text().bold()),
            Span::raw("          Select columns"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  F", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("filter", theme().text().bold()),
            Span::raw("          Filter rows"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  O", theme().text().bold()),
            Span::raw(" or "),
            Span::styled("order", theme().text().bold()),
            Span::raw("           Sort data"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  tabn", theme().text().bold()),
            Span::raw("                  Create new tab with query"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  schema", theme().text().bold()),
            Span::raw("                Show schema browser"),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  reset", theme().text().bold()),
            Span::raw("                 Reset table to original data"),
        ]));

        let pg = Paragraph::new(lines)
            .style(theme().text())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Help")
                    .bottom(
                        StatusBar::new()
                            .mono_color()
                            .centered()
                            .tag(Tag::new(" Close ", " ESC | q ")),
                    )
                    .title_alignment(Alignment::Center)
                    .into_widget(),
            );

        pg.render(area, buf);
    }
}
