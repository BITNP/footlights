#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PositionOption {
    Center,
    Absolute(u32, u32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SizeOption {
    // FitContent(Padding)
    FitContent(u32),
    Absolute(u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicShape {
    shape_type: BasicShapeType,
    position: PositionOption,
    size: SizeOption,
    pub fill: Option<String>,
}

// pub struct ViewBox {
//     content: Box<dyn SvgTangibleObject>,
//     position_option: PositionOption,
//     size_option: SizeOption,
// }

impl BasicShape {
    pub fn new(shape_type: BasicShapeType) -> Self {
        Self {
            shape_type,
            size: SizeOption::Absolute(100, 100),
            position: PositionOption::Center,
            fill: None,
        }
    }
}

impl SizeOptionT for BasicShape {
    fn get_size_option(&self) -> SizeOption {
        self.size
    }
}

impl PositionOptionT for BasicShape {
    fn get_position_option(&self) -> PositionOption {
        self.position
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BasicShapeType {
    Rectangle,
}
