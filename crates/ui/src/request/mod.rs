//! Request builder components
//!
//! Provides UI components for building HTTP requests.

use gpui::*;

pub mod builder;
pub mod method_select;
pub mod url_input;
pub mod headers_editor;
pub mod body_editor;

pub use builder::*;
