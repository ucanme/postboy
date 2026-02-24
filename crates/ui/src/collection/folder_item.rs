//! Folder item component

use gpui::*;

/// Folder item view
pub struct FolderItem;

impl FolderItem {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self
    }
}

impl Render for FolderItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0xccccccff))
            .child("Folder")
    }
}
