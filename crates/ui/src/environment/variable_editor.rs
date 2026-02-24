//! Variable editor component

use gpui::*;

/// Variable editor view
pub struct VariableEditor;

impl VariableEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for VariableEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Variable editor")
    }
}
