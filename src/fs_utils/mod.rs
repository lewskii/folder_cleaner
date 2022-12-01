//! Tools for interacting with the file system.
//! 
//! This module provides miscellaneous tools that make interacting with
//! the file system more convenient.

pub mod error;
mod op;
mod pattern;

#[doc(inline)]
pub use op::remove;
#[doc(inline)]
pub use pattern::FilePattern;
