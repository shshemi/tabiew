pub mod command_pallete;
pub mod data_frame_table;
pub mod error_popup;
pub mod input;
pub mod search_bar;
pub mod sheet;
pub mod status_bar;
pub mod tab_content;
pub mod tabs;
pub mod terminal;
pub mod theme;
mod utils;

pub use tab_content::{Source, TabContentState};
pub use terminal::Terminal;
pub use theme::Styler;

pub mod themes {
    pub use super::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokyoNight};
}
