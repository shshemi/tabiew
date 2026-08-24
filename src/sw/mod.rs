//! State/Widget UI tree — the target architecture replacing `crate::tui`'s
//! `Component` trait (state + view + input fused into one type).
//!
//! Every piece here is split in two:
//!   - a `XyzState` struct: plain data, holds no rendering logic (the Model).
//!   - a `Xyz` struct implementing `ratatui::widgets::StatefulWidget`: a pure
//!     `render(self, area, buf, &mut XyzState)` function (the View).
//!
//! State mutation happens centrally, driven by `AppAction` variants matched
//! in a single dispatcher (the Update), not by each widget handling its own
//! key events. That plumbing (`Context`, `KeyHandler`, `AppAction`) lands once
//! enough leaf widgets exist to need it.
//!
//! This module is built up incrementally alongside `crate::tui`; once it
//! covers the whole tree, `main.rs` switches over and `crate::tui` /
//! `crate::app` / the `Component` trait are deleted.

pub mod app_default;
pub mod pickers;
pub mod widgets;
