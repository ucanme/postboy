//! Dialog component
//!
//! Provides modal dialogs for user interactions.

use gpui::*;

/// Dialog view
pub struct Dialog {
    // TODO: Add state for dialog content
}

impl Dialog {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for Dialog {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgba(0x2d2d2dff))
            .border_1()
            .border_color(rgba(0x444444ff))
            .rounded_lg()
            .p_6()
            .child("Dialog")
    }
}
