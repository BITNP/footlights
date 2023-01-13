use anyhow::Result;
use elementtree::Element;
use image::io::Reader as ImageReader;
mod adt;

pub struct UserInput {}

fn main() -> Result<()> {
    // assets/input.png
    let intput_file = "./assets/output-demo.svg";
    // read file as Vec<u8>
    let input_bytes = std::fs::read(intput_file)?;

    let svg_string = String::from_utf8(input_bytes)?;

    Ok(())
}
