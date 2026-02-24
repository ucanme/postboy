//! Sidebar component for displaying collections
//!
//! Provides a tree view of collections and requests.

use gpui::*;

/// Sidebar view showing collections and requests
pub struct Sidebar {
    // TODO: Add state for collection tree
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for Sidebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .p_3()
                    .border_b_1()
                    .border_color(rgba(0x333333ff))
                    .text_color(rgba(0xccccccff))
                    .child("Collections")
            )
            .child(
                div()
                    .flex_1()
                    .p_2()
                    .text_color(rgba(0x666666ff))
                    .child("No collections yet")
            )
    }
}
