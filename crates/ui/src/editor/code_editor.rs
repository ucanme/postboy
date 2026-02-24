//! Code editor component
//!
//! Provides a code editor with syntax highlighting.

use gpui::*;

/// Code editor view
pub struct CodeEditor {
    // TODO: Add state for editor content
}

impl CodeEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for CodeEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgba(0x1a1a1aff))
            .p_4()
            .text_color(rgba(0xccccccff))
            .child("Code editor")
    }
}
