//! Headers editor component

use gpui::*;

/// Headers editor view
pub struct HeadersEditor;

impl HeadersEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for HeadersEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Headers editor")
    }
}
