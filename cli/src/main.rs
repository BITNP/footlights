use std::path::Path;
use base64::Engine;
use clap::Parser;

use anyhow::Result;
use footlights_engine::configs::{
    structure::{ImageSizeProvider, Structure},
    style::StyleCollection,
};

mod svg_render;

use tokio::io::{stdin, AsyncRead, AsyncReadExt};

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

struct CliImageSizeProvider;

impl ImageSizeProvider for CliImageSizeProvider {
    fn get_image_size(&self, src: &str) -> (u32, u32) {
        if Path::new(src).exists() {
            // case 1: src is a path to a file.
            image::image_dimensions(src).unwrap()
        } else if src.starts_with("http") {
            // case 2: src is a url.
            panic!("Not implemented yet. (case 2)");
        } else if src.starts_with("data:image") {
            // case 3: src is a base64 string.
            panic!("Not implemented yet. (case 3)");
        } else {
            panic!("Invalid image source: {}", src);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = UserInput::parse();

    // 1. Try read data from stdin.
    let mut buffer = Vec::new();
    let mut stdin = tokio::io::stdin();
    stdin.read_buf(&mut buffer).await.unwrap();

    // read yaml file from args[1]
    let yaml = std::fs::read_to_string(args.config)?;

    let mut tt = tinytemplate::TinyTemplate::new();

    tt.add_template("test", &yaml)?;

    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if buffer.is_empty() {
        // 2. Encode the image into data URLs.
        let mut buf = Vec::new();
        // make sure we'll have a slice big enough for base64 + padding
        buf.resize(buffer.len() * 4 / 3 + 4, 0);
        let encode =
            base64::engine::general_purpose::STANDARD_NO_PAD.encode_slice(&buffer, &mut buf)?;

        let data_url = format!("data:image/png;base64,{}", encode);
        map.insert("image".to_string(), data_url);
    }

    println!("{:?}", map);

    tt.render("test", &map)?;

    let styles: StyleCollection = serde_yaml::from_str(&yaml)?;
    let structure = Structure::default();

    let canvas = structure.build_canvas(styles, CliImageSizeProvider{})?;

    let svg_string = canvas.to_svg_string()?;

    let pixmap = svg_render::svg_string_to_pixmap(&svg_string)?;

    pixmap.save_png(&args.output).unwrap();

    Ok(())
}
