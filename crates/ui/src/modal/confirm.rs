//! Confirmation dialog component

use gpui::*;

/// Confirm dialog view
pub struct ConfirmDialog;

impl ConfirmDialog {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for ConfirmDialog {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0xccccccff))
            .child("Confirm dialog")
    }
}
