use adt::build_abstract_dom_tree;
use anyhow::Result;
use image::io::Reader as ImageReader;

use crate::adt::Canvas;
mod adt;
mod svg_render;

pub struct UserInput {}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage:\n\tfootlights <input-style.yml> <output-png>");
        return Ok(());
    }

    let tree = adt::build_abstract_dom_tree();
    let svg_string = tree.to_svg_string()?;
    println!("{}", svg_string);

    // read yaml file from args[1]
    let yaml = std::fs::read_to_string(&args[1])?;
    let root: Canvas = serde_yaml::from_str(&yaml)?;

    // let pixmap = svg_render::svg_string_to_pixmap(&svg_string)?;
    let pixmap = svg_render::svg_string_to_pixmap(&root.to_svg_string()?)?;

    pixmap.save_png(&args[2]).unwrap();
    Ok(())
}
