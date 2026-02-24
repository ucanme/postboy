//! Postboy UI components built with GPUI
//!
//! This crate provides all UI components for the Postboy application.
//! Designed for offline-first operation with a clean, modern interface.

pub mod theme;
pub mod layout;

use gpui::*;

pub use theme::Colors;

/// Initialize the UI module
pub fn init(_cx: &mut Context<()>) {
    // Theme initialization (currently a no-op)
}
