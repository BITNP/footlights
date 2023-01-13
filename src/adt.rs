/// Abstract DOM Tree
use anyhow::Result;
use elementtree::Element;

#[derive(Debug, Clone, Copy, Default)]
pub struct Size(pub u32, pub u32);

#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub u32, pub u32);

pub trait SvgObject {
    fn to_svg(&self) -> Element;
}

pub trait SizeOptionT {
    fn get_size_option(&self) -> SizeOption;
}

pub trait PositionOptionT {
    fn get_position_option(&self) -> PositionOption;
}

trait SvgTangibleObject: SizeOptionT + PositionOptionT {
    fn cal_position(&self, parent_size: Size, size: Size) -> Position {
        let position_option = self.get_position_option();
        match position_option {
            PositionOption::Center => {
                let x = (parent_size.0 as f32 - size.0 as f32) / 2.;
                let y = (parent_size.1 as f32 - size.1 as f32) / 2.;
                Position(x as u32, y as u32)
            }
        }
    }
    fn cal_size(&self, child_size: Size) -> Size {
        let size_option = self.get_size_option();
        match size_option {
            SizeOption::FitContent(padding) => {
                let width = child_size.0 + padding * 2;
                let height = child_size.1 + padding * 2;
                Size(width, height)
            }
            SizeOption::Absolute(width, height) => Size(width, height),
        }
    }
    fn to_svg(&self, size: Size, position: Position) -> Element;
}

#[derive(Debug, Clone, Copy)]
pub enum PositionOption {
    Center,
}

#[derive(Debug, Clone, Copy)]
pub enum SizeOption {
    // FitContent(Padding)
    FitContent(u32),
    Absolute(u32, u32),
}

pub struct BasicShape {
    shape_type: BasicShapeType,
    position: PositionOption,
    size: SizeOption,
}

pub struct ViewBox {
    content: Box<dyn SvgTangibleObject>,
    position_option: PositionOption,
    size_option: SizeOption,
}

impl BasicShape {
    pub fn new(shape_type: BasicShapeType) -> Self {
        Self {
            shape_type,
            size: SizeOption::Absolute(100, 100),
            position: PositionOption::Center,
        }
    }
}

impl SvgTangibleObject for BasicShape {
    fn to_svg(&self, size: Size, position: Position) -> Element {
        let mut element = Element::new("rect");
        element.set_attr("width", size.0.to_string());
        element.set_attr("height", size.1.to_string());
        element.set_attr("x", position.0.to_string());
        element.set_attr("y", position.1.to_string());
        element
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

pub enum BasicShapeType {
    Rectangle,
}

/// A background layer is a layer only contains style.
/// It has no position info.
pub struct Background {
    bg_type: BackgroundType,
    colors: Vec<Color>,
    size: Size,
}

pub struct Color;

pub enum BackgroundType {}

pub struct Image {}

pub struct Fill {}

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

    fn build_svg_canvas(&self, size: Size) -> Element {
        let mut root = elementtree::Element::new(("http://www.w3.org/2000/svg", "svg"));
        root.set_attr("width", &size.0.to_string());
        root.set_attr("height", &size.1.to_string());
        root
    }

    /// Calculate absolute size
    fn calculate_size(&mut self) {}

    fn calculate_position(&mut self) {}

    pub fn to_svg_string(&mut self) -> Result<String> {
        Ok(self.to_svg().to_string()?)
    }
}

impl SvgObject for Canvas {
    fn to_svg(&self) -> Element {
        let mut child_size = Size::default();
        let mut parent_size = Size::default();
        let mut parent_position = Position(0, 0);
        // Calculate the size from the top to the bottom.
        let layers: Vec<_> = self
            .layers
            .iter()
            .rev()
            .map(|o| {
                let size = o.cal_size(child_size);
                child_size = size;
                (o, size)
            })
            // Calculate the position from the bottom to the top.
            .rev()
            .map(|(o, s)| {
                let position = o.cal_position(parent_size, s);
                parent_size = s;
                parent_position = position;
                (o, s, position)
            })
            .collect();

        let mut root = self.build_svg_canvas(child_size);

        layers.iter().for_each(|(o, s, p)| {
            let element = o.to_svg(*s, *p);
            root.append_child(element);
        });

        root
    }
}

#[cfg(test)]
mod tests {
    use crate::layers::Canvas;

    use super::*;

    fn compare_svg_text(left: &str, right: &str) -> Result<()> {
        let left_root = elementtree::Element::from_reader(left.as_bytes()).unwrap();

        let right_root = elementtree::Element::from_reader(right.as_bytes()).unwrap();

        let left_text = left_root.to_string()?;
        let right_text = right_root.to_string()?;
        assert_eq!(left_text, right_text);

        Ok(())
    }

    fn compare_svg(left: &Element, right: &str) -> Result<()> {
        let right_root = elementtree::Element::from_reader(right.as_bytes()).unwrap();

        let left_text = left.to_string()?;
        let right_text = right_root.to_string()?;
        assert_eq!(left_text, right_text);

        Ok(())
    }

    #[test]
    fn svg_basic_shape_rect() -> Result<()> {
        let img = BasicShape::new(BasicShapeType::Rectangle);
        let xml = img.to_svg(Size(100, 100), Position(0, 0));

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
