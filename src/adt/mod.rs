#![warn(missing_docs)]

//! Abstract DOM Tree

pub(crate) mod foundation;
pub(crate) mod svg;

pub(crate) mod background;
pub(crate) mod image;
pub(crate) mod text;

#[cfg(test)] mod tests;

use self::svg::Canvas;
use self::background::Background;
use self::foundation::Color;
use self::image::Image;


pub fn build_abstract_dom_tree() -> Canvas {
    let mut canvas = Canvas::new();

    let background = Background::new_linear_gradient(
        vec![
            (Color("#000000".to_string()), "0%".to_string()),
            (Color("#ffffff".to_string()), "100%".to_string()),
        ],
        45.0,
    );
    canvas.add_layer_on_top(Box::new(background));

    // let mut shape = BasicShape::new(BasicShapeType::Rectangle);
    // shape.size = SizeOption::Absolute(200, 200);
    // shape.fill = Some("blue".to_string());
    // canvas.add_layer_on_top(Box::new(shape));
    //
    // let mut shape2 = BasicShape::new(BasicShapeType::Rectangle);
    // shape2.size = SizeOption::Absolute(200, 200);
    // shape2.position = PositionOption::Absolute(50, 50);
    // shape2.fill = Some("red".to_string());
    // canvas.add_layer_on_top(Box::new(shape2));

    let img = Image::new_from_path("./assets/input.png".to_string());
    canvas.add_layer_on_top(Box::new(img));

    canvas
}
