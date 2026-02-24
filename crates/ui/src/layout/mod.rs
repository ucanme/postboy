//! Layout components for Postboy
//!
//! Provides the main application window and its layout structure.

use gpui::*;

pub mod main_window;
// pub mod sidebar;  // TODO: Update to new API
// pub mod header;    // TODO: Update to new API
// pub mod status_bar; // TODO: Update to new API

pub use main_window::*;

/// Main window dimensions
pub const WINDOW_MIN_WIDTH: f32 = 1200.0;
pub const WINDOW_MIN_HEIGHT: f32 = 700.0;
pub const WINDOW_DEFAULT_WIDTH: f32 = 1400.0;
pub const WINDOW_DEFAULT_HEIGHT: f32 = 900.0;

/// Layout sizing constants
pub mod sizing {
    /// Width of the sidebar
    pub const SIDEBAR_WIDTH: f32 = 280.0;

    /// Width of the request panel (when split)
    pub const REQUEST_PANEL_MIN_WIDTH: f32 = 400.0;

    /// Minimum height of request/response panels
    pub const PANEL_MIN_HEIGHT: f32 = 200.0;

    /// Header height
    pub const HEADER_HEIGHT: f32 = 48.0;

    /// Status bar height
    pub const STATUS_BAR_HEIGHT: f32 = 24.0;

    /// Panel divider size
    pub const DIVIDER_SIZE: f32 = 1.0;

    /// Panel divider hit area (for resize)
    pub const DIVIDER_HIT_AREA: f32 = 8.0;
}

/// Initialize the layout module
pub fn init(_cx: &mut Context<()>) {
    // Layout initialization if needed
}
