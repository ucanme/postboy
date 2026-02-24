//! Response viewer components
//!
//! Provides UI components for viewing HTTP responses.

use gpui::*;

pub mod viewer;
pub mod body_viewer;
pub mod headers_viewer;
pub mod status_viewer;

pub use viewer::*;
