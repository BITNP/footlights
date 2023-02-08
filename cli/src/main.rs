use base64::Engine;
use clap::Parser;
use image::io::Reader;
use std::{io::Cursor, path::Path};

use anyhow::Result;
use footlights_engine::configs::{
    structure::{ImageSizeProvider, Structure},
    style::StyleCollection,
};

mod svg_render;

use tokio::io::{stdin, AsyncRead, AsyncReadExt, AsyncWriteExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct UserInput {
    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    output: Option<String>,

    /// Read data from stdin.
    #[arg(long)]
    stdin: bool,

    /// Output data to stdout.
    #[arg(long)]
    stdout: bool,

    #[arg(short, long)]
    image: Option<String>,

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
            // Case 3: `src` is a base64 string.
            let src = src.split(',').last().unwrap();
            let bytes = base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(src.as_bytes())
                .unwrap();

            let img = Reader::new(Cursor::new(&bytes))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            (img.width(), img.height())
        } else {
            panic!("Invalid image source: {}", src);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = UserInput::parse();

    let mut tt = tinytemplate::TinyTemplate::new();
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if args.stdin {
        // 1. Try read data from stdin.
        let mut buffer = Vec::new();
        let mut stdin = tokio::io::stdin();
        stdin.read_to_end(&mut buffer).await.unwrap();
        // 2. Encode the image into data URLs.
        let encoded = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&buffer);

        let data_url = format!("data:image/png;base64,{}", encoded);
        map.insert("image".to_string(), data_url);
    } else if let Some(image) = args.image {
        map.insert("image".to_string(), image);
    }

    // read yaml file from args[1]
    let yaml = std::fs::read_to_string(args.config)?;

    tt.add_template("style_collections", &yaml)?;
    let rendered_yaml = tt.render("style_collections", &map)?;

    if args.debug {
        eprintln!("{}", rendered_yaml);
    }

    let styles: StyleCollection = serde_yaml::from_str(&rendered_yaml)?;
    let structure = Structure::default();

    let canvas = structure.build_canvas(&styles, CliImageSizeProvider {})?;

    let svg_string = canvas.to_svg_string()?;

    let pixmap = svg_render::svg_string_to_pixmap(&svg_string)?;

    // Output data to stdout.
    if args.stdout {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(&pixmap.encode_png()?).await?;
    }

    // Write data to a file.
    if let Some(output) = args.output {
        pixmap.save_png(&output).unwrap();
    }

    Ok(())
}
