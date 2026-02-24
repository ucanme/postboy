//! Collection tree view component
//!
//! Displays collections and requests in a hierarchical tree.

use gpui::*;

/// Collection tree view
pub struct CollectionTreeView {
    // TODO: Add state for collections
}

impl CollectionTreeView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for CollectionTreeView {
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
                    .child("No collections")
            )
    }
}
