//! Theme definitions for Postboy UI
//!
//! Provides color palettes, typography, and design tokens
//! for both light and dark themes.

use gpui::{rgba, Rgba};

/// Theme mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }
}

/// Color palette
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    // Primary colors
    pub primary: Rgba,
    pub primary_hover: Rgba,
    pub primary_active: Rgba,

    // Accent colors
    pub accent: Rgba,
    pub accent_blue: Rgba,
    pub accent_green: Rgba,
    pub accent_yellow: Rgba,
    pub accent_orange: Rgba,
    pub accent_red: Rgba,
    pub accent_purple: Rgba,

    // Background
    pub background: Rgba,
    pub background_elevated: Rgba,
    pub background_sunken: Rgba,

    // Surface
    pub surface: Rgba,
    pub surface_hover: Rgba,
    pub surface_active: Rgba,

    // Border
    pub border: Rgba,
    pub border_focus: Rgba,
    pub border_muted: Rgba,

    // Text
    pub text: Rgba,
    pub text_muted: Rgba,
    pub text_placeholder: Rgba,
    pub text_disabled: Rgba,
    pub text_inverse: Rgba,

    // Status
    pub success: Rgba,
    pub warning: Rgba,
    pub error: Rgba,
    pub info: Rgba,

    // HTTP method colors
    pub method_get: Rgba,
    pub method_post: Rgba,
    pub method_put: Rgba,
    pub method_delete: Rgba,
    pub method_patch: Rgba,
    pub method_head: Rgba,
    pub method_options: Rgba,
}

impl Colors {
    /// Dark theme color palette (default)
    pub fn dark() -> Self {
        Self {
            // Primary
            primary: rgba(0x007accff),
            primary_hover: rgba(0x1f8ad2ff),
            primary_active: rgba(0x0060a0ff),

            // Accent
            accent: rgba(0x007accff),
            accent_blue: rgba(0x4ec9b0ff),
            accent_green: rgba(0x89d185ff),
            accent_yellow: rgba(0xdcaca5ff),
            accent_orange: rgba(0xce9178ff),
            accent_red: rgba(0xf44747ff),
            accent_purple: rgba(0xb07bf8ff),

            // Background
            background: rgba(0x1e1e1eff),
            background_elevated: rgba(0x252526ff),
            background_sunken: rgba(0x1a1a1aff),

            // Surface
            surface: rgba(0x2d2d30ff),
            surface_hover: rgba(0x3c3c3cff),
            surface_active: rgba(0x404040ff),

            // Border
            border: rgba(0x404040ff),
            border_focus: rgba(0x007accff),
            border_muted: rgba(0x3c3c3cff),

            // Text
            text: rgba(0xccccccff),
            text_muted: rgba(0x858585ff),
            text_placeholder: rgba(0x6a6a6aff),
            text_disabled: rgba(0x4c4c4cff),
            text_inverse: rgba(0x1e1e1eff),

            // Status
            success: rgba(0x89d185ff),
            warning: rgba(0xdcaca5ff),
            error: rgba(0xf44747ff),
            info: rgba(0x4ec9b0ff),

            // HTTP methods
            method_get: rgba(0x4ec9b0ff),
            method_post: rgba(0x569cd6ff),
            method_put: rgba(0xdcdcaaff),
            method_delete: rgba(0xf44747ff),
            method_patch: rgba(0xce9178ff),
            method_head: rgba(0x808080ff),
            method_options: rgba(0x808080ff),
        }
    }

    /// Light theme color palette
    pub fn light() -> Self {
        Self {
            // Primary
            primary: rgba(0x0066ccff),
            primary_hover: rgba(0x0052a3ff),
            primary_active: rgba(0x0078f0ff),

            // Accent
            accent: rgba(0x0066ccff),
            accent_blue: rgba(0x007accff),
            accent_green: rgba(0x4caf50ff),
            accent_yellow: rgba(0xffc107ff),
            accent_orange: rgba(0xff9800ff),
            accent_red: rgba(0xf44336ff),
            accent_purple: rgba(0x9c27b0ff),

            // Background
            background: rgba(0xffffffff),
            background_elevated: rgba(0xf5f5f5ff),
            background_sunken: rgba(0xfafafaff),

            // Surface
            surface: rgba(0xffffffff),
            surface_hover: rgba(0xf0f0f0ff),
            surface_active: rgba(0xe8e8e8ff),

            // Border
            border: rgba(0xe0e0e0ff),
            border_focus: rgba(0x0066ccff),
            border_muted: rgba(0xd0d0d0ff),

            // Text
            text: rgba(0x333333ff),
            text_muted: rgba(0x666666ff),
            text_placeholder: rgba(0x999999ff),
            text_disabled: rgba(0xbdbdbdff),
            text_inverse: rgba(0xffffffff),

            // Status
            success: rgba(0x4caf50ff),
            warning: rgba(0xffc107ff),
            error: rgba(0xf44336ff),
            info: rgba(0x2196f3ff),

            // HTTP methods
            method_get: rgba(0x008000ff),
            method_post: rgba(0x0066ccff),
            method_put: rgba(0xff8c00ff),
            method_delete: rgba(0xdc143cff),
            method_patch: rgba(0xff6347ff),
            method_head: rgba(0x696969ff),
            method_options: rgba(0x696969ff),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::dark()
    }
}

/// Typography settings
#[derive(Debug, Clone)]
pub struct Typography {
    /// Font family for UI text
    pub ui_font_family: &'static str,

    /// Font family for code/monospace text
    pub mono_font_family: &'static str,

    /// Base font size in pixels
    pub base_size: f32,

    /// Small font size
    pub small_size: f32,

    /// Large font size
    pub large_size: f32,

    /// Line height multiplier
    pub line_height: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            ui_font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif",
            mono_font_family: "'Menlo', 'Monaco', 'Courier New', monospace",
            base_size: 13.0,
            small_size: 11.0,
            large_size: 16.0,
            line_height: 1.5,
        }
    }
}

/// Spacing scale
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub unit: f32,
    pub xxs: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            unit: 4.0,
            xxs: 4.0,   // 1x
            xs: 8.0,    // 2x
            sm: 12.0,   // 3x
            md: 16.0,   // 4x
            lg: 24.0,   // 6x
            xl: 32.0,   // 8x
            xxl: 48.0,  // 12x
        }
    }
}

/// Border radius
#[derive(Debug, Clone, Copy)]
pub struct Radius {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub full: f32,
}

impl Default for Radius {
    fn default() -> Self {
        Self {
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            full: 9999.0,
        }
    }
}

/// Complete theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: Colors,
    pub typography: Typography,
    pub spacing: Spacing,
    pub radius: Radius,
}

impl Theme {
    /// Create a new dark theme
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: Colors::dark(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            radius: Radius::default(),
        }
    }

    /// Create a new light theme
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: Colors::light(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            radius: Radius::default(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Get the current theme (can be made dynamic later)
pub fn get_theme() -> Theme {
    Theme::dark()
}

/// Get method color for HTTP method highlighting
pub fn get_method_color(method: &str) -> Rgba {
    let theme = get_theme();
    match method.to_uppercase().as_str() {
        "GET" => theme.colors.method_get,
        "POST" => theme.colors.method_post,
        "PUT" => theme.colors.method_put,
        "DELETE" => theme.colors.method_delete,
        "PATCH" => theme.colors.method_patch,
        "HEAD" => theme.colors.method_head,
        "OPTIONS" => theme.colors.method_options,
        _ => theme.colors.text_muted,
    }
}

/// Get status color for HTTP status codes
pub fn get_status_color(status: u16) -> Rgba {
    let theme = get_theme();
    match status {
        200..299 => theme.colors.success,
        300..399 => theme.colors.info,
        400..499 => theme.colors.warning,
        500..599 => theme.colors.error,
        _ => theme.colors.text_muted,
    }
}

/// Initialize the theme system
pub fn init(_cx: &mut gpui::Context<()>) {
    // Theme initialization can be done here
    // For now, we use the default dark theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_colors() {
        let theme = Theme::dark();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(theme.colors.primary, rgba(0x007accff));
    }

    #[test]
    fn test_light_theme_colors() {
        let theme = Theme::light();
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_eq!(theme.colors.background, rgba(0xffffffff));
    }

    #[test]
    fn test_method_colors() {
        assert_eq!(get_method_color("GET"), rgba(0x4ec9b0ff));
        assert_eq!(get_method_color("POST"), rgba(0x569cd6ff));
        assert_eq!(get_method_color("DELETE"), rgba(0xf44747ff));
    }

    #[test]
    fn test_status_colors() {
        assert_eq!(get_status_color(200), rgba(0x89d185ff)); // success
        assert_eq!(get_status_color(301), rgba(0x4ec9b0ff)); // info
        assert_eq!(get_status_color(404), rgba(0xdcaca5ff)); // warning
        assert_eq!(get_status_color(500), rgba(0xf44747ff)); // error
    }
}
