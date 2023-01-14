use anyhow::Result;
use resvg::{
    tiny_skia::{self, Pixmap},
    usvg,
    usvg_text_layout::{fontdb, TreeTextToPath},
};
/// from svg string to png
pub fn svg_string_to_pixmap(svg_string: &str) -> Result<Pixmap> {
    let opt = usvg::Options::default();
    let mut tree = usvg::Tree::from_data(svg_string.as_bytes(), &opt).unwrap();

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    tree.convert_text(&fontdb, opt.keep_named_groups);

    let pixmap_size = tree.size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    Ok(pixmap)
}
