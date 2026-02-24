//! HTTP method selector component

use gpui::*;

/// Method selector view
pub struct MethodSelector;

impl MethodSelector {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for MethodSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0xccccccff))
            .child("GET")
    }
}
