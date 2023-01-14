use adt::build_abstract_dom_tree;
use clap::Parser;

use anyhow::Result;
use image::io::Reader as ImageReader;

use crate::adt::Canvas;
mod adt;
mod svg_render;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct UserInput {
    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    output: String,

    #[arg(long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = UserInput::parse();

    // read yaml file from args[1]
    let yaml = std::fs::read_to_string(args.config)?;
    let root: Canvas = serde_yaml::from_str(&yaml)?;

    // let pixmap = svg_render::svg_string_to_pixmap(&svg_string)?;
    let pixmap = svg_render::svg_string_to_pixmap(&root.to_svg_string()?)?;

    pixmap.save_png(&args.output).unwrap();
    Ok(())
}
