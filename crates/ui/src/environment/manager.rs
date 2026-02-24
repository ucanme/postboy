//! Environment manager component
//!
//! Manages environments and variables.

use gpui::*;

/// Environment manager view
pub struct EnvironmentManager {
    // TODO: Add state for environments
}

impl EnvironmentManager {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for EnvironmentManager {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .text_color(rgba(0x888888ff))
            .child("Environment manager")
    }
}
