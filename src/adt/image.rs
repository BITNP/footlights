use elementtree::Element;
use serde::{Deserialize, Serialize};

use super::foundation::{Size, SizeOptionT, SizeOption, Position, PositionOptionT, PositionOption};
use super::svg::SvgTangibleObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    path: String,
}

impl Image {
    pub fn new_from_path(path: String) -> Self {
        Self {
            path: path,
        }
    }
}


impl SizeOptionT for Image {
    fn get_size_option(&self) -> SizeOption {
        match imagesize::size(&self.path) {
            Ok(dim) => SizeOption::Absolute(dim.width as u32, dim.height as u32),
            Err(why) => panic!("Error getting dimensions of {:?}: {:?}", self.path, why),
        }
    }
}

impl PositionOptionT for Image {
    fn get_position_option(&self) -> PositionOption {
        PositionOption::Center
    }
}

#[typetag::serde]
impl SvgTangibleObject for Image {
    fn to_svg(&self, size: Size, position: Position) -> (Element, Option<Element>) {
        let mut element = Element::new("image");
        element.set_attr("width", size.0.to_string());
        element.set_attr("height", size.1.to_string());
        element.set_attr("x", position.0.to_string());
        element.set_attr("y", position.1.to_string());
        element.set_attr("href", self.path.clone());

        (element, None)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use super::*;
    use super::super::tests::compare_svg;

    #[test]
    fn svg_image() -> Result<()> {
        let img = Image::new_from_path("./assets/input.png".to_string());

        let (xml, defs) = img.to_svg(Size(100, 100), Position(0, 0));

        assert!(defs.is_none());

        const EXPECT: &str = r#"
        <image width="100" height="100" x="0" y="0" href="./assets/input.png"/>
        "#;
        compare_svg(&xml, EXPECT)?;

        Ok(())
    }
}
