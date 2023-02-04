#![warn(missing_docs)]

//! Abstract DOM Tree

pub mod configs;

pub mod svg;
pub mod foundation;
pub mod shape;
pub mod background;
pub mod image;

pub use svg::Canvas;

#[cfg(test)]
mod tests;
