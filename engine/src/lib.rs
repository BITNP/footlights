#![warn(missing_docs)]

//! Abstract DOM Tree

pub mod foundation;
pub mod svg;

pub mod background;
pub mod image;

pub use svg::Canvas;

#[cfg(test)]
mod tests;
