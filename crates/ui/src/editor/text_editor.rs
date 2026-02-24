//! Text editor component

use gpui::*;

/// Text editor view
pub struct TextEditor;

impl TextEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for TextEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0xccccccff))
            .child("Text editor")
    }
}
