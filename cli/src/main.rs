use base64::Engine;
use clap::Parser;

use anyhow::Result;
use footlights_engine::Canvas;

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

    let root: Canvas = serde_yaml::from_str(&yaml)?;

    // let pixmap = svg_render::svg_string_to_pixmap(&svg_string)?;
    let pixmap = svg_render::svg_string_to_pixmap(&root.to_svg_string()?)?;

    pixmap.save_png(&args.output).unwrap();
    Ok(())
}
