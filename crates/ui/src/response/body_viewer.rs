//! Response body viewer component

use gpui::*;

/// Response body viewer view
pub struct BodyViewer;

impl BodyViewer {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for BodyViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Response body")
    }
}
