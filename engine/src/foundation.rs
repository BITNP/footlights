use serde::{Deserialize, Serialize};

use crate::configs::style::{PositionOption, SizeOption};

#[derive(Debug, Clone, Copy, Default)]
/// Size is a tuple of width and height.
pub struct Size(pub u32, pub u32);

impl From<(u32, u32)> for Size {
    fn from((w, h): (u32, u32)) -> Self {
        Self(w, h)
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// Position is a tuple of x and y.
pub struct Position(pub u32, pub u32);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Color is a string of color.
///
/// Example: "red", "#ff0000", "rgb(255, 0, 0)"
pub struct Color(pub String);

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// A trait for objects that can get their configed size option.
pub trait SizeOptionT {
    /// Get the size option of the object.
    fn get_size_option(&self) -> SizeOption;
}

/// A trait for objects that can get their configed position option.
pub trait PositionOptionT {
    /// Get the position option of the object.
    fn get_position_option(&self) -> PositionOption;
}
