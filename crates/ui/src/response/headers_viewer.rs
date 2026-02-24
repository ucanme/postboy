//! Response headers viewer component

use gpui::*;

/// Headers viewer view
pub struct HeadersViewer;

impl HeadersViewer {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for HeadersViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Response headers")
    }
}
