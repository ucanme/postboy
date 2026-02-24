//! Status bar component for the main window
//!
//! Displays application status and information.

use gpui::*;

/// Status bar view
pub struct StatusBar {
    // TODO: Add state for status messages, etc.
}

impl StatusBar {
    /// Create a new status bar
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .child(
                div()
                    .text_color(rgba(0x888888ff))
                    .child("Ready")
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_color(rgba(0x666666ff))
                    .child("No requests sent")
            )
    }
}
