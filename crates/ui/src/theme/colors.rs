//! Color definitions for Postboy UI
//!
//! Defines a comprehensive color palette supporting light and dark themes.

use gpui::{rgb, Rgba};

/// Primary color palette for Postboy
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    // Base colors
    pub background: Rgba,
    pub surface: Rgba,
    pub panel: Rgba,
    pub border: Rgba,

    // Text colors
    pub text_primary: Rgba,
    pub text_secondary: Rgba,
    pub text_muted: Rgba,
    pub text_disabled: Rgba,
    pub text_inverse: Rgba,

    // Accent colors
    pub primary: Rgba,
    pub primary_hover: Rgba,
    pub secondary: Rgba,
    pub success: Rgba,
    pub warning: Rgba,
    pub error: Rgba,
    pub info: Rgba,

    // HTTP method colors (for request badges)
    pub method_get: Rgba,
    pub method_post: Rgba,
    pub method_put: Rgba,
    pub method_delete: Rgba,
    pub method_patch: Rgba,
    pub method_head: Rgba,
    pub method_options: Rgba,

    // Status colors
    pub status_2xx: Rgba,   // Success
    pub status_3xx: Rgba,   // Redirect
    pub status_4xx: Rgba,   // Client error
    pub status_5xx: Rgba,   // Server error

    // UI element colors
    pub focus_ring: Rgba,
    pub hover_background: Rgba,
    pub background_sunken: Rgba,
    pub selection: Rgba,
    pub scrollbar: Rgba,
    pub scrollbar_hover: Rgba,

    // Syntax highlighting colors (for code editor)
    pub syntax_keyword: Rgba,
    pub syntax_string: Rgba,
    pub syntax_number: Rgba,
    pub syntax_comment: Rgba,
    pub syntax_function: Rgba,
    pub syntax_variable: Rgba,
    pub syntax_operator: Rgba,
    pub syntax_property: Rgba,
}

impl Colors {
    /// Get dark theme colors
    pub fn dark() -> Self {
        Self {
            // Base colors - VS Code Dark inspired
            background: rgb(0x1e1e1e),
            surface: rgb(0x252526),
            panel: rgb(0x2d2d30),
            border: rgb(0x3c3c3c),

            // Text colors
            text_primary: rgb(0xcccccc),
            text_secondary: rgb(0x9e9e9e),
            text_muted: rgb(0x6e6e6e),
            text_disabled: rgb(0x4e4e4e),
            text_inverse: rgb(0xffffff),

            // Accent colors
            primary: rgb(0x007acc),     // Blue
            primary_hover: rgb(0x1f8ad2),
            secondary: rgb(0x6a9955),   // Green
            success: rgb(0x4ec9b0),     // Green-ish
            warning: rgb(0xce9178),     // Orange
            error: rgb(0xf44747),       // Red
            info: rgb(0x75beff),        // Light blue

            // HTTP method colors
            method_get: rgb(0x4ec9b0),
            method_post: rgb(0x569cd6),
            method_put: rgb(0xdcdcaa),
            method_delete: rgb(0xf44747),
            method_patch: rgb(0xce9178),
            method_head: rgb(0x808080),
            method_options: rgb(0x808080),

            // Status colors
            status_2xx: rgb(0x4ec9b0),
            status_3xx: rgb(0xce9178),
            status_4xx: rgb(0xf44747),
            status_5xx: rgb(0xf14c4c),

            // UI element colors
            focus_ring: Self::rgba_with_alpha(0x00, 0x7a, 0xcc, 0.3),
            hover_background: rgb(0x2a2d2e),
            background_sunken: rgb(0x1a1a1a),
            selection: Self::rgba_with_alpha(0x26, 0x4f, 0x78, 0.4),
            scrollbar: rgb(0x424242),
            scrollbar_hover: rgb(0x4f4f4f),

            // Syntax highlighting
            syntax_keyword: rgb(0x569cd6),
            syntax_string: rgb(0xce9178),
            syntax_number: rgb(0xb5cea8),
            syntax_comment: rgb(0x6a9955),
            syntax_function: rgb(0xdcdcaa),
            syntax_variable: rgb(0x9cdcfe),
            syntax_operator: rgb(0xd4d4d4),
            syntax_property: rgb(0x9cdcfe),
        }
    }

    /// Get light theme colors
    pub fn light() -> Self {
        Self {
            // Base colors
            background: rgb(0xffffff),
            surface: rgb(0xf3f3f3),
            panel: rgb(0xf8f8f8),
            border: rgb(0xe0e0e0),

            // Text colors
            text_primary: rgb(0x333333),
            text_secondary: rgb(0x666666),
            text_muted: rgb(0x999999),
            text_disabled: rgb(0xbdbdbd),
            text_inverse: rgb(0x000000),

            // Accent colors
            primary: rgb(0x0066cc),
            primary_hover: rgb(0x0052a3),
            secondary: rgb(0x4a7c59),
            success: rgb(0x3da86a),
            warning: rgb(0xcc6b34),
            error: rgb(0xd63031),
            info: rgb(0x3594dc),

            // HTTP method colors
            method_get: rgb(0x3da86a),
            method_post: rgb(0x0066cc),
            method_put: rgb(0xb38800),
            method_delete: rgb(0xcc3300),
            method_patch: rgb(0x996600),
            method_head: rgb(0x666666),
            method_options: rgb(0x666666),

            // Status colors
            status_2xx: rgb(0x3da86a),
            status_3xx: rgb(0xcc6b34),
            status_4xx: rgb(0xd63031),
            status_5xx: rgb(0xf14c4c),

            // UI element colors
            focus_ring: Self::rgba_with_alpha(0x00, 0x66, 0xcc, 0.3),
            hover_background: rgb(0xf0f0f0),
            background_sunken: rgb(0xfafafa),
            selection: Self::rgba_with_alpha(0xad, 0xd6, 0xff, 0.3),
            scrollbar: rgb(0xc0c0c0),
            scrollbar_hover: rgb(0xa0a0a0),

            // Syntax highlighting
            syntax_keyword: rgb(0x0000ff),
            syntax_string: rgb(0xa31515),
            syntax_number: rgb(0x098658),
            syntax_comment: rgb(0x008000),
            syntax_function: rgb(0x795e26),
            syntax_variable: rgb(0x001080),
            syntax_operator: rgb(0x000000),
            syntax_property: rgb(0x001080),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::dark()
    }
}

impl Colors {
    /// Get color for HTTP method
    pub fn method_color(&self, method: &str) -> Rgba {
        match method.to_uppercase().as_str() {
            "GET" => self.method_get,
            "POST" => self.method_post,
            "PUT" => self.method_put,
            "DELETE" => self.method_delete,
            "PATCH" => self.method_patch,
            "HEAD" => self.method_head,
            "OPTIONS" => self.method_options,
            _ => self.text_secondary,
        }
    }

    /// Get color for HTTP status code
    pub fn status_color(&self, status: u16) -> Rgba {
        match status {
            200..=299 => self.status_2xx,
            300..=399 => self.status_3xx,
            400..=499 => self.status_4xx,
            500..=599 => self.status_5xx,
            _ => self.text_muted,
        }
    }

    /// Create a semi-transparent version of a color
    pub fn with_alpha(&self, color: Rgba, alpha: f32) -> Rgba {
        color.with_alpha(alpha)
    }

    /// Create a new Rgba with alpha
    pub fn rgba_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba { r, g, b, a }
    }
}

/// Theme type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn colors(self) -> Colors {
        match self {
            Theme::Dark => Colors::dark(),
            Theme::Light => Colors::light(),
        }
    }
}

// Helper function for creating Rgba with explicit alpha
trait RgbaExt {
    fn with_alpha(self, alpha: f32) -> Rgba;
}

impl RgbaExt for Rgba {
    fn with_alpha(self, alpha: f32) -> Rgba {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }
}

/// RGB color structure compatible with GPUI
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.a = alpha;
        self
    }
}

impl From<Rgba> for gpui::Rgba {
    fn from(rgba: Rgba) -> Self {
        gpui::Rgba {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
            a: rgba.a,
        }
    }
}

/// Create an RGB color (alpha = 1.0)
pub const fn rgb(r: u32) -> Rgba {
    let r = ((r >> 16) & 0xFF) as f32 / 255.0;
    let g = ((r >> 8) & 0xFF) as f32 / 255.0;
    let b = (r & 0xFF) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

/// Create an RGBA color from hex
pub const fn rgba(r: u32, g: u32, b: u32, a: f32) -> Rgba {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_creation() {
        let color = rgb(0xFF0000); // Red
        assert!((color.r - 1.0).abs() < 0.01);
        assert!(color.g < 0.01);
        assert!(color.b < 0.01);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_method_colors() {
        let colors = Colors::dark();
        assert_eq!(colors.method_color("GET"), colors.method_get);
        assert_eq!(colors.method_color("POST"), colors.method_post);
        assert_eq!(colors.method_color("DELETE"), colors.method_delete);
    }

    #[test]
    fn test_status_colors() {
        let colors = Colors::dark();
        assert_eq!(colors.status_color(200), colors.status_2xx);
        assert_eq!(colors.status_color(301), colors.status_3xx);
        assert_eq!(colors.status_color(404), colors.status_4xx);
        assert_eq!(colors.status_color(500), colors.status_5xx);
    }
}
