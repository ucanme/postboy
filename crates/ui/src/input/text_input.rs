//! Text input component for Postboy
//!
//! A reusable text input field with full keyboard editing support.

use gpui::*;
use std::sync::Arc;

/// Text input view
pub struct TextInput {
    /// Current text content
    text: String,
    /// Placeholder text
    placeholder: String,
    /// Is focused
    focused: bool,
    /// Cursor position
    cursor: usize,
    /// Selection start
    selection_start: Option<usize>,
    /// Selection end
    selection_end: Option<usize>,
    /// On change callback
    on_change: Option<Arc<dyn Fn(&str, &mut ViewContext<TextInput>) + 'static>>,
    /// On commit callback (Enter key)
    on_commit: Option<Arc<dyn Fn(&str, &mut ViewContext<TextInput>) + 'static>>,
}

impl TextInput {
    /// Create a new text input
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            text: String::new(),
            placeholder: String::new(),
            focused: false,
            cursor: 0,
            selection_start: None,
            selection_end: None,
            on_change: None,
            on_commit: None,
        }
    }

    /// Set the initial text
    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;
        self.cursor = text.chars().count();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }

    /// Set callback for text changes
    pub fn on_change(
        mut self,
        callback: impl Fn(&str, &mut ViewContext<TextInput>) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Set callback for commit (Enter key)
    pub fn on_commit(
        mut self,
        callback: impl Fn(&str, &mut ViewContext<TextInput>) + 'static,
    ) -> Self {
        self.on_commit = Some(Arc::new(callback));
        self
    }

    /// Get current text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Handle character input
    fn handle_char(&mut self, c: char, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.get_selection_range() {
            // Delete selected text first
            self.delete_range(selection);
        }

        let mut chars = self.text.chars().collect::<Vec<_>>();
        chars.insert(self.cursor, c);
        self.text = chars.into_iter().collect();
        self.cursor += 1;
        self.notify_change(cx);
    }

    /// Handle backspace
    fn handle_backspace(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.get_selection_range() {
            // Delete selection
            self.delete_range(selection);
            self.notify_change(cx);
            return;
        }

        if self.cursor > 0 {
            let mut chars = self.text.chars().collect::<Vec<_>>();
            chars.remove(self.cursor - 1);
            self.text = chars.into_iter().collect();
            self.cursor -= 1;
            self.notify_change(cx);
        }
    }

    /// Handle delete key
    fn handle_delete(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.get_selection_range() {
            // Delete selection
            self.delete_range(selection);
            self.notify_change(cx);
            return;
        }

        let mut chars = self.text.chars().collect::<Vec<_>>();
        if self.cursor < chars.len() {
            chars.remove(self.cursor);
            self.text = chars.into_iter().collect();
            self.notify_change(cx);
        }
    }

    /// Handle arrow keys
    fn handle_arrow(&mut self, direction: ArrowDirection, cx: &mut ViewContext<Self>) {
        match direction {
            ArrowDirection::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            ArrowDirection::Right => {
                let len = self.text.chars().count();
                if self.cursor < len {
                    self.cursor += 1;
                }
            }
            ArrowDirection::Home => {
                self.cursor = 0;
            }
            ArrowDirection::End => {
                self.cursor = self.text.chars().count();
            }
        }
        cx.notify();
    }

    /// Handle enter key
    fn handle_enter(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(callback) = &self.on_commit {
            callback(&self.text, cx);
        }
    }

    /// Get selection range if any
    fn get_selection_range(&self) -> Option<(usize, usize)> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (s, e) = if start <= end { (start, end) } else { (end, start) };
            if s != e {
                return Some((s, e));
            }
        }
        None
    }

    /// Delete a range of characters
    fn delete_range(&mut self, range: (usize, usize)) {
        let (start, end) = range;
        let mut chars = self.text.chars().collect::<Vec<_>>();
        for _ in start..end {
            if start < chars.len() {
                chars.remove(start);
            }
        }
        self.text = chars.into_iter().collect();
        self.cursor = start;
        self.selection_start = None;
        self.selection_end = None;
    }

    /// Notify text change
    fn notify_change(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(callback) = &self.on_change {
            callback(&self.text, cx);
        } else {
            cx.notify();
        }
    }
}

#[derive(Clone, Copy)]
enum ArrowDirection {
    Left,
    Right,
    Home,
    End,
}

impl Render for TextInput {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display_text = if self.text.is_empty() {
            self.placeholder.clone()
        } else {
            self.text.clone()
        };

        let text_color = if self.text.is_empty() {
            rgba(0x9CA3AFFF) // Gray for placeholder
        } else {
            rgba(0x1F2937FF) // Dark gray for text
        };

        div()
            .w_full()
            .h(px(40.0))
            .px_4()
            .flex()
            .items_center()
            .bg(rgba(0xF9FAFBFF))
            .border_2()
            .border_color(if self.focused {
                rgba(0x3B82F6FF) // Blue when focused
            } else {
                rgba(0xE5E7EBFF) // Gray when not focused
            })
            .rounded(px(8.0))
            .cursor(CursorStyle::Text)
            .on_click(cx.listener(|this, _event, cx| {
                this.focused = true;
                // Move cursor to end
                this.cursor = this.text.chars().count();
                cx.notify();
            }))
            .on_focus(cx.listener(|this, _event, cx| {
                this.focused = true;
                cx.notify();
            }))
            .on_down(cx.listener(|this, event, cx| {
                // Handle keyboard input
                if let Some keystroke) = event.keystroke.as_ref() {
                    match keystroke.key.as_str() {
                        "backspace" => {
                            this.handle_backspace(cx);
                        }
                        "delete" => {
                            this.handle_delete(cx);
                        }
                        "enter" => {
                            this.handle_enter(cx);
                        }
                        "left" => {
                            this.handle_arrow(ArrowDirection::Left, cx);
                        }
                        "right" => {
                            this.handle_arrow(ArrowDirection::Right, cx);
                        }
                        "home" => {
                            this.handle_arrow(ArrowDirection::Home, cx);
                        }
                        "end" => {
                            this.handle_arrow(ArrowDirection::End, cx);
                        }
                        "escape" => {
                            this.focused = false;
                            cx.notify();
                        }
                        _ => {
                            // Handle character input
                            if let Some(c) = keystroke.key.chars().next() {
                                if !keystroke.modifiers.control_or_command()
                                    && !keystroke.modifiers.alt()
                                    && c.is_ascii()
                                    && !c.is_ascii_control()
                                {
                                    this.handle_char(c, cx);
                                }
                            }
                        }
                    }
                }
            }))
            .on_blur(cx.listener(|this, _event, cx| {
                this.focused = false;
                cx.notify();
            }))
            .child(
                div()
                    .flex_1()
                    .text_color(text_color)
                    .text_size(px(14.0))
                    .overflow_x_hidden()
                    .whitespace_nowrap()
                    .child(display_text)
            )
    }
}
