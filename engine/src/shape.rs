use elementtree::Element;
use serde::{Deserialize, Serialize};

use crate::{
    configs::style::{PositionOption, SizeOption},
    foundation::{Position, PositionOptionT, Size, SizeOptionT},
    svg::SvgTangibleObject,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicShape {
    shape_type: BasicShapeType,
    position: PositionOption,
    size: SizeOption,
    pub fill: Option<String>,
}

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

impl SvgTangibleObject for BasicShape {
    fn to_svg(&self, size: Size, position: Position) -> (Element, Option<Element>) {
        let mut element = Element::new("rect");
        element.set_attr("width", size.0.to_string());
        element.set_attr("height", size.1.to_string());
        element.set_attr("x", position.0.to_string());
        element.set_attr("y", position.1.to_string());
        // TODO: valid css color
        self.fill
            .as_ref()
            .map(|fill| element.set_attr("fill", fill));
        // self.stroke
        //     .as_ref()
        //     .map(|stroke| element.set_attr("stroke", stroke));
        (element, None)
    }
}
