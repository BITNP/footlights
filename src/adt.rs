#![warn(missing_docs)]
/// Abstract DOM Tree
use anyhow::Result;
use elementtree::Element;

#[derive(Debug, Clone, Copy, Default)]
pub struct Size(pub u32, pub u32);

#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub u32, pub u32);

#[derive(Debug, Clone, Default)]
pub struct Color(String);

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub trait SvgObject {
    fn to_svg(&self) -> Element;
}

pub trait SizeOptionT {
    fn get_size_option(&self) -> SizeOption;
}

pub trait PositionOptionT {
    fn get_position_option(&self) -> PositionOption;
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

#[derive(Debug, Clone, Copy)]
pub enum PositionOption {
    Center,
    Absolute(u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub enum SizeOption {
    // FitContent(Padding)
    FitContent(u32),
    Absolute(u32, u32),
}

#[derive(Debug, Clone)]
pub struct BasicShape {
    shape_type: BasicShapeType,
    position: PositionOption,
    size: SizeOption,
    fill: Option<String>,
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
            fill: None,
        }
    }
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

#[derive(Debug, Clone)]
pub enum BasicShapeType {
    Rectangle,
}

/// A background layer is a layer only contains style.
/// It has no position info.
//TODO: use BackgroundType directly?
#[derive(Debug, Clone)]
pub struct Background {
    bg_type: BackgroundType,
    // size: SizeOption,
}

#[derive(Debug, Clone)]
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
        let mut linear_gradient = LinearGradient { stops, degree };

        Self {
            bg_type: BackgroundType::Linear(linear_gradient),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearGradient {
    /// Color, offset
    stops: Vec<(Color, String)>,
    degree: f32,
}

#[derive(Debug, Clone)]
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
                let mut linear = Element::new("linearGradient");
                linear.set_attr("id", "background");

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

                let mut element = Element::new("rect");
                element.set_attr("width", size.0.to_string());
                element.set_attr("height", size.1.to_string());
                element.set_attr("x", position.0.to_string());
                element.set_attr("y", position.1.to_string());
                element.set_attr("fill", "url(#background)");

                (element, Some(linear))
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

    pub fn to_svg_string(&self) -> Result<String> {
        Ok(self.to_svg().to_string()?)
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

pub fn build_abstract_dom_tree() -> Canvas {
    let mut canvas = Canvas::new();

    let mut shape = BasicShape::new(BasicShapeType::Rectangle);
    shape.size = SizeOption::Absolute(200, 200);
    shape.fill = Some("blue".to_string());

    let mut shape2 = BasicShape::new(BasicShapeType::Rectangle);
    shape2.size = SizeOption::Absolute(200, 200);
    shape2.position = PositionOption::Absolute(50, 50);
    shape2.fill = Some("red".to_string());

    let background = Background::new_linear_gradient(
        vec![
            (Color("#000000".to_string()), "0%".to_string()),
            (Color("#ffffff".to_string()), "100%".to_string()),
        ],
        45.0,
    );

    canvas.add_layer_on_top(Box::new(background));
    canvas.add_layer_on_top(Box::new(shape));
    canvas.add_layer_on_top(Box::new(shape2));
    canvas
}

#[cfg(test)]
mod tests {

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
        let mut background = Background::new_linear_gradient(stops, 45.0);
        let (xml, defs) = background.to_svg(Size(100, 100), Position(0, 0));

        const EXPECT_DEFS: &str = r#"
        <linearGradient id="background"><stop offset="0" stop-color="red"/><stop offset="1" stop-color="blue"/></linearGradient>
        "#;

        const EXPECT: &str = r#"
        <rect width="100" height="100" x="0" y="0" fill="url(#background)" />
        "#;

        compare_svg(&xml, EXPECT).unwrap();
        compare_svg(&defs.unwrap(), EXPECT_DEFS).unwrap();
        Ok(())
    }
}
