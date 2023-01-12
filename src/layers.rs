use anyhow::Result;
use elementtree::Element;

#[derive(Debug, Clone, Copy)]
pub struct Size(pub u32, pub u32);

pub enum Layer {
    Background(Background),
    Image(Image),
    BasicShape(BasicShape),
}

impl Layer {
    pub fn to_xml_tree(&self) -> Element {
        match self {
            Layer::Background(_) => todo!(),
            Layer::Image(i) => todo!(),
            Layer::BasicShape(s) => s.to_xml_tree(),
        }
    }

    pub fn get_size(&self) -> Size {
        match self {
            Layer::Background(_) => todo!(),
            Layer::Image(_) => todo!(),
            Layer::BasicShape(s) => s.get_size(),
        }
    }
}

impl From<Background> for Layer {
    fn from(background: Background) -> Self {
        Layer::Background(background)
    }
}

impl From<Image> for Layer {
    fn from(image: Image) -> Self {
        Layer::Image(image)
    }
}

impl From<BasicShape> for Layer {
    fn from(basic_shape: BasicShape) -> Self {
        Layer::BasicShape(basic_shape)
    }
}

pub struct BasicShape {
    shape_type: BasicShapeType,
    size: Size,
}

impl BasicShape {
    pub fn new(shape_type: BasicShapeType) -> Self {
        Self {
            shape_type,
            size: Size(100, 100),
        }
    }

    pub fn to_xml_tree(&self) -> Element {
        let mut element = Element::new("rect");
        element.set_attr("width", self.size.0.to_string());
        element.set_attr("height", self.size.1.to_string());
        element
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

pub enum BasicShapeType {
    Rectangle,
}

pub struct Background {
    bg_type: BackgroundType,
    colors: Vec<Color>,
}

pub struct Color;

pub enum BackgroundType {}

pub struct Image {}

pub struct Fill {}

pub struct Canvas {
    layers: Vec<Layer>,
    canvas: (u32, u32),
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            canvas: (0, 0),
        }
    }

    pub fn add_layer_on_top<L: Into<Layer>>(&mut self, layer: L) {
        self.layers.push(layer.into());
    }

    pub fn add_layer_on_bottom(&mut self, layer: Layer) {
        self.layers.insert(0, layer);
    }

    fn build_canvas(&self) -> Element {
        let mut root = elementtree::Element::new(("http://www.w3.org/2000/svg", "svg"));
        root.set_attr("width", &self.canvas.0.to_string());
        root.set_attr("height", &self.canvas.1.to_string());
        root
    }

    pub fn build_xml(&self) -> Result<String> {
        let mut canvas = self.build_canvas();

        for layer in self.layers.iter() {
            let ele = layer.to_xml_tree();
            canvas.append_child(ele);
        }

        Ok(canvas.to_string()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::layers::Canvas;

    use super::*;

    // TODO: pub fn parse_xml
    fn compare_xml_text(left: &str, right: &str) -> Result<()> {
        let left_root = elementtree::Element::from_reader(left.as_bytes()).unwrap();

        let right_root = elementtree::Element::from_reader(right.as_bytes()).unwrap();

        let left_text = left_root.to_string()?;
        let right_text = right_root.to_string()?;
        assert_eq!(left_text, right_text);

        Ok(())
    }

    #[test]
    fn basic_shape_rect() -> Result<()> {
        let img = BasicShape::new(BasicShapeType::Rectangle);
        let xml = img.to_xml_tree().to_string()?;

        const EXPECT: &str = r#"
        <rect width="100" height="100"/>
        "#;
        compare_xml_text(&xml, EXPECT).unwrap();

        Ok(())
    }

    #[test]
    fn canvas() -> Result<()> {
        let img = Canvas::new();
        let xml = img.build_xml()?;
        const EXPECT: &str = r#"
<svg width="0" height="0" xmlns="http://www.w3.org/2000/svg"></svg>
"#;

        compare_xml_text(&xml, EXPECT).unwrap();

        Ok(())
    }

    #[test]
    fn svg_gen() -> Result<()> {
        let mut canvas = Canvas::new();
        let shape1 = BasicShape::new(BasicShapeType::Rectangle);
        canvas.add_layer_on_top(shape1);

        const EXPECT: &str = r#"
        <svg width="0" height="0" xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100"/></svg>
        "#;
        let xml = canvas.build_xml()?;

        compare_xml_text(&xml, EXPECT).unwrap();

        Ok(())
    }
}
