use anyhow::Result;
use elementtree::Element;

use crate::configs::style::{PositionOption, SizeOption};

use super::foundation::{Position, PositionOptionT, Size, SizeOptionT};

pub trait SvgObject {
    fn to_svg(&self) -> Element;
}

pub trait SvgTangibleObject: SizeOptionT + PositionOptionT + std::fmt::Debug {
    fn cal_position(&self, parent_size: Size, size: Size) -> Position {
        let position_option = self.get_position_option();
        match position_option {
            PositionOption::Center => {
                let x = (parent_size.0 as f32 - size.0 as f32) / 2.;
                let y = (parent_size.1 as f32 - size.1 as f32) / 2.;
                Position(x as u32, y as u32)
            }
            PositionOption::Absolute(x, y) => Position(x, y),
        }
    }
    fn cal_size(&self, child_size: Size) -> Size {
        let size_option = self.get_size_option();
        println!("size_option: {:?}", size_option);
        match size_option {
            SizeOption::FitContent(padding) => {
                let width = child_size.0 + padding * 2;
                let height = child_size.1 + padding * 2;
                Size(width, height)
            }
            SizeOption::Absolute(width, height) => Size(width, height),
        }
    }
    /// Generate svg elements with the given size and position.
    ///
    /// * `size`:
    /// * `position`:
    fn to_svg(&self, size: Size, position: Position) -> (Element, Option<Element>);
}

/// A canvas is a container for a series of layers.
pub struct Canvas {
    /// A series of layers that are rendered in order.
    ///
    /// The first layer is rendered first, and the last layer is rendered last.
    layers: Vec<Box<dyn SvgTangibleObject>>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer_on_top(&mut self, layer: Box<dyn SvgTangibleObject>) {
        self.layers.push(layer);
    }

    // pub fn add_layer_on_bottom(&mut self, layer: ViewBox) {
    //     self.layers.insert(0, layer);
    // }

    pub fn build_svg_canvas(&self, size: Size) -> Element {
        let mut root = elementtree::Element::new(("http://www.w3.org/2000/svg", "svg"));
        root.set_attr("width", &size.0.to_string());
        root.set_attr("height", &size.1.to_string());
        root
    }

    /// Calculate absolute size
    fn calculate_size(&mut self) {}

    fn calculate_position(&mut self) {}

    pub fn to_svg_string(&self) -> Result<String> {
        let string = self.to_svg().to_string()?;

        println!("{}", string);
        Ok(string)
    }
}

impl SvgObject for Canvas {
    fn to_svg(&self) -> Element {
        let mut child_size = Size::default();
        let mut parent_size = Size::default();
        let mut parent_position = Position(0, 0);
        println!("layers: {:?}", self.layers);
        // Calculate the size from the top to the bottom.
        let (childs, defs_childs): (Vec<_>, Vec<_>) = self
            .layers
            .iter()
            .enumerate()
            .rev()
            .map(|(i, o)| {
                let size = o.cal_size(child_size);
                println!(
                    "i: {}, o: {:?}, size: {:?}, child_size: {:?}",
                    i, o, size, child_size
                );
                child_size = size;
                (i, o, size)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            // Calculate the position from the bottom to the top.
            .map(|(i, o, s)| {
                let position = o.cal_position(parent_size, s);
                parent_size = s;
                parent_position = position;
                (i, o, s, position)
            })
            .map(|(_, o, s, p)| o.to_svg(s, p))
            .unzip();

        let mut root = self.build_svg_canvas(child_size);
        let mut defs = Element::new("defs");

        childs.into_iter().for_each(|child| {
            root.append_child(child);
        });

        defs_childs.into_iter().for_each(|child| {
            if let Some(child) = child {
                defs.append_child(child);
            }
        });

        if defs.child_count() != 0 {
            root.append_child(defs);
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use crate::shape::{BasicShape, BasicShapeType};

    use super::super::foundation::*;
    use super::super::tests::compare_svg;
    use super::*;

    #[test]
    fn svg_basic_shape_rect() -> Result<()> {
        let img = BasicShape::new(BasicShapeType::Rectangle);
        let (xml, defs) = img.to_svg(Size(100, 100), Position(0, 0));

        assert!(defs.is_none());

        const EXPECT: &str = r#"
        <rect width="100" height="100" x="0" y="0"/>
        "#;
        compare_svg(&xml, EXPECT).unwrap();

        Ok(())
    }

    #[test]
    fn svg_canvas() -> Result<()> {
        let img = Canvas::new();
        let xml = img.build_svg_canvas(Size::default());
        const EXPECT: &str = r#"
    <svg width="0" height="0" xmlns="http://www.w3.org/2000/svg"></svg>
    "#;

        compare_svg(&xml, EXPECT).unwrap();

        Ok(())
    }

    #[test]
    fn svg_gen() -> Result<()> {
        let mut canvas = Canvas::new();
        let shape1 = BasicShape::new(BasicShapeType::Rectangle);
        canvas.add_layer_on_top(Box::new(shape1));

        const EXPECT: &str = r#"
        <svg width="100" height="100" xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100" x="0" y="0"/></svg>
        "#;
        let xml = canvas.to_svg();

        compare_svg(&xml, EXPECT).unwrap();

        Ok(())
    }
}
