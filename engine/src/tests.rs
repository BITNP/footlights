use anyhow::Result;
use elementtree::Element;

use super::background::Background;
use super::foundation::Color;
use super::image::Image;
use super::svg::Canvas;

pub fn compare_svg_text(left: &str, right: &str) -> Result<()> {
    let left_root = elementtree::Element::from_reader(left.as_bytes()).unwrap();

    let right_root = elementtree::Element::from_reader(right.as_bytes()).unwrap();

    let left_text = left_root.to_string()?;
    let right_text = right_root.to_string()?;
    assert_eq!(left_text, right_text);

    Ok(())
}

pub fn compare_svg(left: &Element, right: &str) -> Result<()> {
    let mut right_root: Element = elementtree::Element::from_reader(right.as_bytes()).unwrap();
    delete_all_whitespaces(&mut right_root);

    let left_text = left.to_string()?;
    let right_text = right_root.to_string()?;
    assert_eq!(left_text, right_text);

    Ok(())
}

pub fn delete_all_whitespaces(tree: &mut Element) {
    if !tree.text().is_empty() {
        tree.set_text(tree.text().trim().to_string());
    }

    if !tree.tail().is_empty() {
        tree.set_tail(tree.tail().trim().to_string());
    }

    for child in tree.children_mut() {
        delete_all_whitespaces(child);
    }
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