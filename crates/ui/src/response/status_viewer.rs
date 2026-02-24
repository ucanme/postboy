//! Response status viewer component

use gpui::*;

/// Status viewer view
pub struct StatusViewer;

impl StatusViewer {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for StatusViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Status")
    }
}
