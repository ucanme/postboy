//! URL input component

use gpui::*;

/// URL input view
pub struct UrlInput;

impl UrlInput {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for UrlInput {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Enter URL...")
    }
}
