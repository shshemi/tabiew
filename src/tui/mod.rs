pub mod status_bar;
pub mod tabs;
pub mod tabular;
pub mod terminal;
pub mod theme;
mod utils;
pub mod widget;

pub use tabular::{Tabular, TabularType};
pub use terminal::Terminal;
pub use theme::Styler;

pub mod themes {
    pub use super::theme::{Argonaut, Monokai, Terminal};
}
