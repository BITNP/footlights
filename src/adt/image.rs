use elementtree::Element;
use serde::{Deserialize, Serialize};

use super::foundation::{Position, PositionOption, PositionOptionT, Size, SizeOption, SizeOptionT};
use super::svg::SvgTangibleObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    path: String,
    round: Option<usize>,
}

impl Image {
    pub fn new_from_path(path: String) -> Self {
        Self { path, round: None }
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
        let mut svg = Element::new("svg");
        svg.set_attr("width", size.0.to_string());
        svg.set_attr("height", size.1.to_string());
        svg.set_attr("x", position.0.to_string());
        svg.set_attr("y", position.1.to_string());

        let mut defs = Element::new("defs");
        let mut clip_path = Element::new("clipPath");
        clip_path.set_attr("id", "clip");

        let mut rect = Element::new("rect");
        rect.set_attr("width", "100%");
        rect.set_attr("height", "100%");
        rect.set_attr("rx", self.round.unwrap_or(0).to_string());

        clip_path.append_child(rect);

        defs.append_child(clip_path);
        svg.append_child(defs);
        let mut img = Element::new("image");
        img.set_attr("href", self.path.clone());
        img.set_attr("width", "100%");
        img.set_attr("height", "100%");
        img.set_attr("clip-path", "url(#clip)");
        svg.append_child(img);

        (svg, None)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::compare_svg;
    use super::*;
    use anyhow::Result;

    #[test]
    fn svg_image() -> Result<()> {
        let mut img = Image::new_from_path("./assets/input.png".to_string());
        img.round = Some(15);

        let (xml, defs) = img.to_svg(Size(100, 100), Position(0, 0));

        assert!(defs.is_none());

        const EXPECT: &str = r#"
<svg x="0" y="0" height="100" width="100">
    <defs>
        <clipPath id="clip">
            <rect width="100%" height="100%" rx="15" />
        </clipPath>
    </defs>
    <image height="100%" href="./assets/input.png" width="100%" clip-path="url(#clip)" />
</svg>
        "#;
        compare_svg(&xml, EXPECT)?;

        Ok(())
    }
}
