use anyhow::Result;
use elementtree::Element;

use super::foundation::Color;
use super::svg::Canvas;
use super::background::Background;
use super::image::Image;

pub fn compare_svg_text(left: &str, right: &str) -> Result<()> {
    let left_root = elementtree::Element::from_reader(left.as_bytes()).unwrap();

    let right_root = elementtree::Element::from_reader(right.as_bytes()).unwrap();

    let left_text = left_root.to_string()?;
    let right_text = right_root.to_string()?;
    assert_eq!(left_text, right_text);

    Ok(())
}

pub fn compare_svg(left: &Element, right: &str) -> Result<()> {
    let right_root = elementtree::Element::from_reader(right.as_bytes()).unwrap();

    let left_text = left.to_string()?;
    let right_text = right_root.to_string()?;
    assert_eq!(left_text, right_text);

    Ok(())
}


#[test]
fn adt_serialization() -> Result<()> {
    let mut canvas = Canvas::new();

    let background = Background::new_linear_gradient(
        vec![
            (Color("#000000".to_string()), "0%".to_string()),
            (Color("#ffffff".to_string()), "100%".to_string()),
        ],
        45.0,
    );
    canvas.add_layer_on_top(Box::new(background));

    let img = Image::new_from_path("./assets/input.png".to_string());
    canvas.add_layer_on_top(Box::new(img));

    let yaml = serde_yaml::to_string(&canvas).unwrap();
    println!("{}", yaml);

    Ok(())
}
