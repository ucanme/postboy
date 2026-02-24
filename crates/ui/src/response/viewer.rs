//! Response viewer component
//!
//! Main component for viewing HTTP responses.

use gpui::*;

/// Response viewer view
pub struct ResponseViewer {
    // TODO: Add state for response data
}

impl ResponseViewer {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for ResponseViewer {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .p_4()
            .gap_4()
            .child(
                div()
                    .text_color(rgba(0xccccccff))
                    .font_weight(FontWeight::BOLD)
                    .child("Response")
            )
            .child(
                div()
                    .text_color(rgba(0x888888ff))
                    .child("Response viewer will be implemented here")
            )
    }
}
