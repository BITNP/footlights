use adt::build_abstract_dom_tree;
use anyhow::Result;
use image::io::Reader as ImageReader;
mod adt;
mod svg_render;

pub struct UserInput {}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\tfootlights <output-png>");
        return Ok(());
    }

    let tree = adt::build_abstract_dom_tree();
    let svg_string = tree.to_svg_string()?;
    println!("{}", svg_string);
    let pixmap = svg_render::svg_string_to_pixmap(&svg_string)?;

    pixmap.save_png(&args[1]).unwrap();
    Ok(())
}
