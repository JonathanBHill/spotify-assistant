//! Generic paginator processing utilities for rspotify
//!
//! This module provides small, focused examples of how to work with rspotify's
//! paginator streams in four different shapes: a free function, an enum, a trait,
//! and a struct. Each example lives in its own file and is intended to be simple
//! to adopt in other parts of the codebase.

pub mod event;
pub mod runner;
pub mod r#trait;

// Re-export the most commonly used items for convenience.
pub use event::PaginatorEvent;
pub use r#trait::PaginatorProcessor;
pub use runner::PaginatorRunner;
