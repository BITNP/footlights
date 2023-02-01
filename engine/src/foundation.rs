use serde::{Deserialize, Serialize};

use crate::configs::style::{PositionOption, SizeOption};

#[derive(Debug, Clone, Copy, Default)]
pub struct Size(pub u32, pub u32);

#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub u32, pub u32);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Color(pub String);

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub trait SizeOptionT {
    fn get_size_option(&self) -> SizeOption;
}

pub trait PositionOptionT {
    fn get_position_option(&self) -> PositionOption;
}
