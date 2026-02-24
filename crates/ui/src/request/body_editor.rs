//! Request body editor component

use gpui::*;

/// Body editor view
pub struct BodyEditor;

impl BodyEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for BodyEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Body editor")
    }
}
