//! Header component for the main window
//!
//! Provides the top toolbar with navigation and actions.

use gpui::*;

/// Header view
pub struct Header {
    // TODO: Add state for search, actions, etc.
}

impl Header {
    /// Create a new header
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for Header {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .px_4()
            .gap_2()
            .child(
                div()
                    .text_color(rgba(0xccccccff))
                    .font_weight(FontWeight::BOLD)
                    .child("Postboy")
            )
            .child(
                div()
                    .flex_1()
                    .h(px(24.0))
                    .bg(rgba(0x1a1a1aff))
                    .border_1()
                    .border_color(rgba(0x444444ff))
                    .rounded_md()
                    .p_2()
                    .text_color(rgba(0x666666ff))
                    .child("Search...")
            )
    }
}
