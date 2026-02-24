//! Typography system for Postboy UI
//!
//! Provides consistent text styling across the application.

use gpui::{px, Rem, Pixels, Style};
use std::sync::Arc;

/// Font family definitions
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FontFamily {
    /// Monospace font for code and data
    Mono,
    /// System UI font for interface elements
    System,
    /// Proportional font for body text
    Sans,
}

impl FontFamily {
    pub fn css_name(&self) -> &'static str {
        match self {
            FontFamily::Mono => "ui-monospace, SF Mono, Monaco, Menlo, Consolas, monospace",
            FontFamily::System => "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
            FontFamily::Sans => "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
        }
    }
}

/// Text size presets
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextSize {
    /// 11px - Tiny text
    Xs,
    /// 12px - Small text
    Sm,
    /// 13px - Default text
    Base,
    /// 14px - Medium text
    Md,
    /// 16px - Large text
    Lg,
    /// 18px - Extra large text
    Xl,
    /// 20px - Heading text
    Xxl,
    /// 24px - Title text
    Xxxl,
}

impl TextSize {
    pub fn pixels(&self) -> Pixels {
        match self {
            TextSize::Xs => px(11.0),
            TextSize::Sm => px(12.0),
            TextSize::Base => px(13.0),
            TextSize::Md => px(14.0),
            TextSize::Lg => px(16.0),
            TextSize::Xl => px(18.0),
            TextSize::Xxl => px(20.0),
            TextSize::Xxxl => px(24.0),
        }
    }
}

/// Text weight
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl FontWeight {
    pub fn css_value(&self) -> u32 {
        *self as u32
    }
}

/// Text line height
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineHeight {
    Default,
    Tight,
    Relaxed,
    Custom(f32),
}

impl LineHeight {
    pub fn multiplier(&self) -> f32 {
        match self {
            LineHeight::Default => 1.5,
            LineHeight::Tight => 1.25,
            LineHeight::Relaxed => 1.75,
            LineHeight::Custom(v) => *v,
        }
    }
}

/// Typography style builder
#[derive(Clone, Debug)]
pub struct Typography {
    pub family: FontFamily,
    pub size: TextSize,
    pub weight: FontWeight,
    pub line_height: LineHeight,
    pub letter_spacing: Option<f32>,
    pub color: Option<Arc<str>>,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            family: FontFamily::System,
            size: TextSize::Base,
            weight: FontWeight::Normal,
            line_height: LineHeight::Default,
            letter_spacing: None,
            color: None,
        }
    }
}

impl Typography {
    /// Create a new typography style
    pub fn new() -> Self {
        Self::default()
    }

    /// Set font family
    pub fn with_family(mut self, family: FontFamily) -> Self {
        self.family = family;
        self
    }

    /// Set text size
    pub fn with_size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    /// Set font weight
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set line height
    pub fn with_line_height(mut self, line_height: LineHeight) -> Self {
        self.line_height = line_height;
        self
    }

    /// Set letter spacing
    pub fn with_letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = Some(spacing);
        self
    }

    /// Set color
    pub fn with_color(mut self, color: impl Into<Arc<str>>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Apply style to GPUI Style
    pub fn apply_to(&self, style: &mut Style) {
        style.set_font_family(self.family.css_name());
        style.set_font_size(self.size.pixels());
        style.set_font_weight(self.weight.css_value());
        style.set_line_height(self.line_height.multiplier());

        if let Some(spacing) = self.letter_spacing {
            style.set_letter_spacing(px(spacing));
        }

        if let Some(color) = &self.color {
            style.set_color(color.as_ref());
        }
    }
}

/// Common typography presets
pub mod presets {
    use super::*;

    /// Default UI text
    pub fn default() -> Typography {
        Typography::new()
    }

    /// Heading text
    pub fn heading() -> Typography {
        Typography::new()
            .with_size(TextSize::Xl)
            .with_weight(FontWeight::SemiBold)
            .with_line_height(LineHeight::Tight)
    }

    /// Subheading
    pub fn subheading() -> Typography {
        Typography::new()
            .with_size(TextSize::Lg)
            .with_weight(FontWeight::Medium)
            .with_line_height(LineHeight::Tight)
    }

    /// Code/monospace text
    pub fn mono() -> Typography {
        Typography::new()
            .with_family(FontFamily::Mono)
            .with_size(TextSize::Sm)
    }

    /// Small label text
    pub fn label() -> Typography {
        Typography::new()
            .with_size(TextSize::Xs)
            .with_weight(FontWeight::Medium)
            .with_letter_spacing(0.05)
    }

    /// Button text
    pub fn button() -> Typography {
        Typography::new()
            .with_size(TextSize::Base)
            .with_weight(FontWeight::Medium)
    }

    /// HTTP method text
    pub fn method() -> Typography {
        Typography::new()
            .with_family(FontFamily::Mono)
            .with_size(TextSize::Sm)
            .with_weight(FontWeight::SemiBold)
    }

    /// Status code text
    pub fn status_code() -> Typography {
        Typography::new()
            .with_family(FontFamily::Mono)
            .with_size(TextSize::Base)
            .with_weight(FontWeight::Medium)
    }

    /// Body text
    pub fn body() -> Typography {
        Typography::new()
            .with_size(TextSize::Base)
            .with_line_height(LineHeight::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_family_css() {
        assert!(FontFamily::Mono.css_name().contains("monospace"));
        assert!(FontFamily::System.css_name().contains("system-ui"));
    }

    #[test]
    fn test_text_sizes() {
        assert_eq!(TextSize::Xs.pixels(), px(11.0));
        assert_eq!(TextSize::Base.pixels(), px(13.0));
        assert_eq!(TextSize::Xxxl.pixels(), px(24.0));
    }

    #[test]
    fn test_line_height_multiplier() {
        assert_eq!(LineHeight::Default.multiplier(), 1.5);
        assert_eq!(LineHeight::Tight.multiplier(), 1.25);
        assert_eq!(LineHeight::Custom(2.0).multiplier(), 2.0);
    }

    #[test]
    fn test_typography_builder() {
        let typo = Typography::new()
            .with_family(FontFamily::Mono)
            .with_size(TextSize::Lg)
            .with_weight(FontWeight::Bold);

        assert_eq!(typo.family, FontFamily::Mono);
        assert_eq!(typo.size, TextSize::Lg);
        assert_eq!(typo.weight, FontWeight::Bold);
    }
}
