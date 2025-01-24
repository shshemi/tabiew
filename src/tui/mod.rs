pub mod data_frame_table;
pub mod input;
pub mod prompt;
pub mod sheet;
pub mod status_bar;
pub mod search_bar;
pub mod tabs;
pub mod tab_content;
pub mod terminal;
pub mod theme;
pub mod error_popup;
pub mod command_pallete;
mod utils;

pub use tab_content::{TabContentState, Source};
pub use terminal::Terminal;
pub use theme::Styler;

pub mod themes {
    pub use super::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokioNight};
}
