pub mod command_palette;
pub mod data_frame_table;
pub mod enumerated_list;
pub mod error_popup;
pub mod pickers;
pub mod plots;
pub mod schema;
pub mod search_bar;
pub mod sheet;
pub mod status_bar;
pub mod tab;
pub mod tab_content;
pub mod terminal;
pub mod theme;
mod utils;
pub mod widgets;

pub use tab_content::{TabContentState, TableType};
pub use terminal::Terminal;
pub use theme::Styler;

pub mod themes {
    pub use super::theme::{Argonaut, Catppuccin, Monokai, Nord, Terminal, TokyoNight};
}
