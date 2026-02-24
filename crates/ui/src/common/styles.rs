//! Common styles
//!
//! Shared styles and utilities.

use gpui::*;

/// Common style utilities
pub struct Styles;

impl Styles {
    /// Get base text color
    pub fn text_base() -> Rgba {
        rgba(0xccccccff)
    }

    /// Get muted text color
    pub fn text_muted() -> Rgba {
        rgba(0x888888ff)
    }

    /// Get border color
    pub fn border_color() -> Rgba {
        rgba(0x333333ff)
    }

    /// Get background color
    pub fn bg_base() -> Rgba {
        rgba(0x1a1a1aff)
    }

    /// Get panel background color
    pub fn bg_panel() -> Rgba {
        rgba(0x252525ff)
    }
}
