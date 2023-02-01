use elementtree::Element;
use serde::{Deserialize, Serialize};

use crate::configs::style::{PositionOption, SizeOption};

use super::foundation::{Color, Position, PositionOptionT, Size, SizeOptionT};
use super::svg::SvgTangibleObject;

/// A background layer is a layer only contains style.
/// It has no position info.
//TODO: use BackgroundType directly?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    bg_type: BackgroundType,
    // size: SizeOption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundType {
    Pure(Color),
    Linear(LinearGradient),
    Radial(RadialGradient),
}

impl Background {
    pub fn new() -> Self {
        Self {
            bg_type: BackgroundType::Pure(Color("white".to_string())),
        }
    }

    pub fn new_pure(color: Color) -> Self {
        Self {
            bg_type: BackgroundType::Pure(color),
        }
    }

    pub fn new_linear_gradient(stops: Vec<(Color, String)>, degree: f32) -> Self {
        let linear_gradient = LinearGradient { stops, degree };

        Self {
            bg_type: BackgroundType::Linear(linear_gradient),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearGradient {
    /// Color, offset
    stops: Vec<(Color, String)>,
    degree: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialGradient {}

impl SizeOptionT for Background {
    fn get_size_option(&self) -> SizeOption {
        SizeOption::FitContent(100)
    }
}

impl PositionOptionT for Background {
    fn get_position_option(&self) -> PositionOption {
        PositionOption::Center
    }
}

impl SvgTangibleObject for Background {
    fn to_svg(&self, size: Size, position: Position) -> (Element, Option<Element>) {
        match &self.bg_type {
            BackgroundType::Linear(linear_gradient) => {
                let mut svg = Element::new("svg");
                svg.set_attr("width", size.0.to_string());
                svg.set_attr("height", size.1.to_string());
                svg.set_attr("x", position.0.to_string());
                svg.set_attr("y", position.1.to_string());

                let mut defs = Element::new("defs");

                let mut linear = Element::new("linearGradient");
                linear.set_attr("id", "background");
                linear.set_attr(
                    "gradientTransform",
                    format!("rotate({})", linear_gradient.degree),
                );

                linear_gradient
                    .stops
                    .iter()
                    .map(|(color, offset)| {
                        let mut stop = Element::new("stop");
                        stop.set_attr("offset", offset);
                        stop.set_attr("stop-color", color.0.clone());
                        stop
                    })
                    .for_each(|stop| {
                        linear.append_child(stop);
                    });

                let mut rect = Element::new("rect");
                rect.set_attr("width", "100%");
                rect.set_attr("height", "100%");
                rect.set_attr("fill", "url(#background)");

                defs.append_child(linear);
                svg.append_child(defs);
                svg.append_child(rect);

                (svg, None)
            }
            BackgroundType::Radial(_) => todo!(),
            BackgroundType::Pure(color) => {
                let mut element = Element::new("rect");
                element.set_attr("width", size.0.to_string());
                element.set_attr("height", size.1.to_string());
                element.set_attr("x", position.0.to_string());
                element.set_attr("y", position.1.to_string());
                element.set_attr("fill", color.0.clone());

                (element, None)
            }
        }
    }
}

pub struct Fill {}

#[cfg(test)]
mod tests {
    use super::super::tests::compare_svg;
    use super::*;
    use anyhow::Result;

    #[test]
    fn svg_background_pure() -> Result<()> {
        let background = Background::new_pure(Color("red".to_string()));

        let (xml, defs) = background.to_svg(Size(100, 100), Position(0, 0));

        assert!(defs.is_none());

        const EXPECT: &str = r#"
        <rect width="100" height="100" x="0" y="0" fill="red"/>
        "#;
        compare_svg(&xml, EXPECT).unwrap();

        Ok(())
    }

    #[test]
    fn svg_background_linear_gradient() -> Result<()> {
        let stops = vec![
            ("red".into(), 0.0.to_string()),
            ("blue".into(), 1.0.to_string()),
        ];
        let background = Background::new_linear_gradient(stops, 45.0);
        let (xml, _) = background.to_svg(Size(100, 100), Position(0, 0));

        const EXPECT: &str = r#"
<svg x="0" y="0" height="100" width="100">
    <defs>
        <linearGradient gradientTransform="rotate(45)" id="background">
            <stop offset="0" stop-color="red"/>
            <stop offset="1" stop-color="blue"/>
        </linearGradient>
    </defs>
    <rect width="100%" height="100%" fill="url(#background)" />
</svg>
        "#;

        compare_svg(&xml, EXPECT).unwrap();
        Ok(())
    }
}
